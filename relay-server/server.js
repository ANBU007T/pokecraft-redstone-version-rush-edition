// PokéCRAFT Global Cloud Relay Server
// Deployable to Render.com, Fly.io, Railway, Glitch, Heroku, or VPS
// Zero-config multiplayer relay supporting 5-letter room codes worldwide.

const http = require('http');
const { WebSocketServer } = require('ws');

const PORT = process.env.PORT || 5901;
const CODE_CHARS = 'ABCDEFGHJKLMNPQRSTUVWXYZ23456789';

function genRoomCode() {
  let code = '';
  for (let i = 0; i < 5; i++) {
    code += CODE_CHARS[Math.floor(Math.random() * CODE_CHARS.length)];
  }
  return code;
}

function genToken() {
  return Math.random().toString(36).slice(2) + Math.random().toString(36).slice(2);
}

const rooms = new Map();

const server = http.createServer((req, res) => {
  const url = new URL(req.url, `http://${req.headers.host || 'localhost'}`);
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', '*');

  if (req.method === 'OPTIONS') {
    res.writeHead(204);
    res.end();
    return;
  }

  if (url.pathname === '/' || url.pathname === '/health') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      status: 'online',
      service: 'PokéCRAFT Cloud Relay Server',
      rooms: rooms.size
    }));
    return;
  }

  const match = url.pathname.match(/^\/api\/room\/([A-Za-z0-9]+)/);
  if (match) {
    const code = match[1].toUpperCase();
    const room = rooms.get(code);
    res.writeHead(200, { 'Content-Type': 'application/json' });
    res.end(JSON.stringify({
      ok: true,
      code,
      exists: Boolean(room),
      count: room ? (1 + room.guests.size) : 0
    }));
    return;
  }

  res.writeHead(404, { 'Content-Type': 'text/plain' });
  res.end('Not Found');
});

const wss = new WebSocketServer({ noServer: true });

server.on('upgrade', (req, socket, head) => {
  const url = new URL(req.url, `http://${req.headers.host || 'localhost'}`);
  const match = url.pathname.match(/^\/api\/room\/([A-Za-z0-9]+)/);
  if (!match) {
    socket.write('HTTP/1.1 404 Not Found\r\n\r\n');
    socket.destroy();
    return;
  }

  wss.handleUpgrade(req, socket, head, (ws) => {
    wss.emit('connection', ws, req, url, match[1]);
  });
});

wss.on('connection', (ws, req, url, targetCode) => {
  const name = url.searchParams.get('name') || 'STEVE';
  const skin = url.searchParams.get('skin') || (targetCode === 'NEW' ? 'steve' : 'alex');
  const clientBuild = url.searchParams.get('v') || '';
  const resumeToken = url.searchParams.get('resume');

  if (targetCode === 'NEW') {
    let code;
    do { code = genRoomCode(); } while (rooms.has(code));
    const hostToken = genToken();
    const room = {
      code, build: clientBuild, token: hostToken, host: ws,
      guests: new Map(), tokens: new Map(), roster: [[0, name, skin, 0]],
      hostHoldTimer: null, guestHoldTimers: new Map()
    };
    rooms.set(code, room);
    ws.roomCode = code; ws.slot = 0; ws.token = hostToken;

    console.log(`[CloudRelay] Room ${code} hosted by ${name}`);
    ws.send(JSON.stringify({ t: 'welcome', slot: 0, code, token: hostToken, roster: room.roster }));
    setupSocketHandlers(ws, room);
    return;
  }

  const code = targetCode.toUpperCase();
  const room = rooms.get(code);
  if (!room) {
    ws.send(JSON.stringify({ t: 'err', e: 4404 }));
    ws.close(4404, 'nocode');
    return;
  }

  if (room.build && clientBuild && room.build !== clientBuild) {
    ws.send(JSON.stringify({ t: 'err', e: 4426 }));
    ws.close(4426, 'version');
    return;
  }

  if (resumeToken) {
    if (resumeToken === room.token) {
      if (room.hostHoldTimer) clearTimeout(room.hostHoldTimer);
      room.host = ws; ws.roomCode = code; ws.slot = 0; ws.token = resumeToken;
      const hostRoster = room.roster.find((r) => r[0] === 0);
      if (hostRoster) hostRoster[3] = 0;
      ws.send(JSON.stringify({ t: 'welcome', slot: 0, code, token: resumeToken, roster: room.roster }));
      broadcastToGuests(room, JSON.stringify({ t: 'hostback' }));
      setupSocketHandlers(ws, room);
      return;
    }

    for (const [s, tok] of room.tokens.entries()) {
      if (tok === resumeToken) {
        const timer = room.guestHoldTimers.get(s);
        if (timer) clearTimeout(timer);
        room.guests.set(s, ws); ws.roomCode = code; ws.slot = s; ws.token = resumeToken;
        const guestRoster = room.roster.find((r) => r[0] === s);
        if (guestRoster) guestRoster[3] = 0;
        ws.send(JSON.stringify({ t: 'welcome', slot: s, code, token: resumeToken, roster: room.roster }));
        if (room.host && room.host.readyState === 1) room.host.send(JSON.stringify({ t: 'back', slot: s }));
        broadcastToGuests(room, JSON.stringify({ t: 'back', slot: s }), s);
        setupSocketHandlers(ws, room);
        return;
      }
    }
  }

  const availableSlots = [1, 2, 3].filter((s) => !room.guests.has(s) && !room.guestHoldTimers.has(s));
  if (availableSlots.length === 0) {
    ws.send(JSON.stringify({ t: 'err', e: 4403 }));
    ws.close(4403, 'full');
    return;
  }

  const assignedSlot = availableSlots[0];
  const guestToken = genToken();
  room.guests.set(assignedSlot, ws);
  room.tokens.set(assignedSlot, guestToken);
  room.roster.push([assignedSlot, name, skin, 0]);
  ws.roomCode = code; ws.slot = assignedSlot; ws.token = guestToken;

  console.log(`[CloudRelay] Guest ${name} joined room ${code} (slot ${assignedSlot})`);
  ws.send(JSON.stringify({ t: 'welcome', slot: assignedSlot, code, token: guestToken, roster: room.roster }));

  const joinMsg = JSON.stringify({ t: 'join', slot: assignedSlot, name, skin });
  if (room.host && room.host.readyState === 1) room.host.send(joinMsg);
  broadcastToGuests(room, joinMsg, assignedSlot);

  setupSocketHandlers(ws, room);
});

function broadcastToGuests(room, msg, excludeSlot = -1) {
  for (const [slot, gws] of room.guests.entries()) {
    if (slot !== excludeSlot && gws.readyState === 1) gws.send(msg);
  }
}

function setupSocketHandlers(ws, room) {
  ws.on('message', (message, isBinary) => {
    if (!isBinary) {
      const text = message.toString();
      if (text === 'ping') { if (ws.readyState === 1) ws.send('pong'); return; }
      if (text === 'leave') { handleLeave(ws, room); return; }
      return;
    }
    routeBinary(ws, room, message);
  });

  ws.on('close', () => handleClose(ws, room));
}

function routeBinary(ws, room, data) {
  const buf = Buffer.isBuffer(data) ? data : Buffer.from(data);
  const senderSlot = ws.slot;
  let offset = 0;
  const out = { 0: [], 1: [], 2: [], 3: [] };

  while (offset + 6 <= buf.length) {
    const mask = buf[offset];
    const len = buf.readUInt32LE(offset + 2);
    const total = 6 + len;
    if (offset + total > buf.length) break;

    const sub = Buffer.allocUnsafe(total);
    buf.copy(sub, 0, offset, offset + total);
    sub[0] = senderSlot;

    for (let k = 0; k <= 3; k++) {
      if (k !== senderSlot && (mask & (1 << k))) out[k].push(sub);
    }
    offset += total;
  }

  if (out[0].length && room.host && room.host.readyState === 1) {
    room.host.send(Buffer.concat(out[0]));
  }
  for (const [slot, gws] of room.guests.entries()) {
    if (out[slot] && out[slot].length && gws.readyState === 1) {
      gws.send(Buffer.concat(out[slot]));
    }
  }
}

function handleLeave(ws, room) {
  if (ws.slot === 0) {
    broadcastToGuests(room, JSON.stringify({ t: 'hostgone' }));
    for (const [, gws] of room.guests.entries()) try { gws.close(4000); } catch(e){}
    rooms.delete(room.code);
  } else {
    room.guests.delete(ws.slot);
    room.tokens.delete(ws.slot);
    room.roster = room.roster.filter((r) => r[0] !== ws.slot);
    const leaveMsg = JSON.stringify({ t: 'leave', slot: ws.slot });
    if (room.host && room.host.readyState === 1) room.host.send(leaveMsg);
    broadcastToGuests(room, leaveMsg, ws.slot);
  }
}

function handleClose(ws, room) {
  if (!rooms.has(room.code)) return;
  if (ws.slot === 0) {
    if (room.host === ws) {
      const hostRoster = room.roster.find((r) => r[0] === 0);
      if (hostRoster) hostRoster[3] = 1;
      broadcastToGuests(room, JSON.stringify({ t: 'hostwait' }));
      if (room.hostHoldTimer) clearTimeout(room.hostHoldTimer);
      room.hostHoldTimer = setTimeout(() => {
        if (room.host === ws) {
          broadcastToGuests(room, JSON.stringify({ t: 'hostgone' }));
          for (const [, gws] of room.guests.entries()) try { gws.close(4000); } catch(e){}
          rooms.delete(room.code);
        }
      }, 90000);
    }
  } else {
    if (room.guests.get(ws.slot) === ws) {
      const guestRoster = room.roster.find((r) => r[0] === ws.slot);
      if (guestRoster) guestRoster[3] = 1;
      if (room.host && room.host.readyState === 1) room.host.send(JSON.stringify({ t: 'lost', slot: ws.slot }));
      const slot = ws.slot;
      const timer = setTimeout(() => {
        if (room.guests.get(slot) === ws) {
          room.guests.delete(slot);
          room.tokens.delete(slot);
          room.roster = room.roster.filter((r) => r[0] !== slot);
          const leaveMsg = JSON.stringify({ t: 'leave', slot });
          if (room.host && room.host.readyState === 1) room.host.send(leaveMsg);
          broadcastToGuests(room, leaveMsg, slot);
        }
      }, 30000);
      room.guestHoldTimers.set(slot, timer);
    }
  }
}

server.listen(PORT, () => {
  console.log(`[CloudRelay] PokéCRAFT Cloud Relay Server running on port ${PORT}`);
});
