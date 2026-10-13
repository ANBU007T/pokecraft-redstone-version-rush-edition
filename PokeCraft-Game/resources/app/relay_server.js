// PokéCRAFT PC Edition - Embedded WebSocket Room Relay Server
// Implements the PokéCRAFT multiplayer wire protocol for local LAN and online co-op.

const http = require('http');
const { WebSocketServer } = require('ws');
const os = require('os');

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

function getLocalIpAddresses() {
  const ips = [];
  const interfaces = os.networkInterfaces();
  for (const name of Object.keys(interfaces)) {
    for (const iface of interfaces[name]) {
      if (iface.family === 'IPv4' && !iface.internal) {
        ips.push(iface.address);
      }
    }
  }
  return ips.length ? ips : ['127.0.0.1'];
}

// Room store: code -> RoomState
const rooms = new Map();

function createRelayServer(options = {}) {
  const port = options.port || 5901;
  const host = options.host || '0.0.0.0';

  const server = http.createServer((req, res) => {
    const parsedUrl = new URL(req.url, `http://${req.headers.host || 'localhost'}`);
    
    // CORS headers for local/cross-origin requests
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', '*');

    if (req.method === 'OPTIONS') {
      res.writeHead(204);
      res.end();
      return;
    }

    if (parsedUrl.pathname === '/' || parsedUrl.pathname === '/health') {
      res.writeHead(200, { 'Content-Type': 'application/json' });
      res.end(JSON.stringify({
        status: 'ok',
        service: 'PokeCraft Relay Server',
        activeRooms: rooms.size,
        localIps: getLocalIpAddresses(),
        port
      }));
      return;
    }

    // Match /api/room/<code> probe
    const match = parsedUrl.pathname.match(/^\/api\/room\/([A-Za-z0-9]+)/);
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
    const parsedUrl = new URL(req.url, `http://${req.headers.host || 'localhost'}`);
    const match = parsedUrl.pathname.match(/^\/api\/room\/([A-Za-z0-9]+)/);
    if (!match) {
      socket.write('HTTP/1.1 404 Not Found\r\n\r\n');
      socket.destroy();
      return;
    }

    wss.handleUpgrade(req, socket, head, (ws) => {
      wss.emit('connection', ws, req, parsedUrl, match[1]);
    });
  });

  wss.on('connection', (ws, req, parsedUrl, targetCode) => {
    const name = parsedUrl.searchParams.get('name') || 'STEVE';
    const skin = parsedUrl.searchParams.get('skin') || (targetCode === 'NEW' ? 'steve' : 'alex');
    const clientBuild = parsedUrl.searchParams.get('v') || '';
    const resumeToken = parsedUrl.searchParams.get('resume');

    // HOSTING A NEW ROOM
    if (targetCode === 'NEW') {
      let code;
      do {
        code = genRoomCode();
      } while (rooms.has(code));

      const hostToken = genToken();
      const room = {
        code,
        build: clientBuild,
        token: hostToken,
        host: ws,
        guests: new Map(),
        tokens: new Map(),
        roster: [[0, name, skin, 0]],
        hostHoldTimer: null,
        guestHoldTimers: new Map()
      };

      rooms.set(code, room);
      ws.roomCode = code;
      ws.slot = 0;
      ws.token = hostToken;

      console.log(`[Relay] Room created: ${code} by ${name} (Steve)`);

      ws.send(JSON.stringify({
        t: 'welcome',
        slot: 0,
        code,
        token: hostToken,
        roster: room.roster
      }));

      setupSocketHandlers(ws, room);
      return;
    }

    // JOINING OR RESUMING AN EXISTING ROOM
    const code = targetCode.toUpperCase();
    const room = rooms.get(code);

    if (!room) {
      console.log(`[Relay] Join rejected: Room ${code} not found`);
      ws.send(JSON.stringify({ t: 'err', e: 4404 }));
      ws.close(4404, 'nocode');
      return;
    }

    // Check version
    if (room.build && clientBuild && room.build !== clientBuild) {
      console.log(`[Relay] Join rejected: Version mismatch for ${code} (host ${room.build} vs client ${clientBuild})`);
      ws.send(JSON.stringify({ t: 'err', e: 4426 }));
      ws.close(4426, 'version');
      return;
    }

    // Resuming connection
    if (resumeToken) {
      // Host resuming
      if (resumeToken === room.token) {
        if (room.hostHoldTimer) {
          clearTimeout(room.hostHoldTimer);
          room.hostHoldTimer = null;
        }
        room.host = ws;
        ws.roomCode = code;
        ws.slot = 0;
        ws.token = resumeToken;

        const hostRoster = room.roster.find((r) => r[0] === 0);
        if (hostRoster) hostRoster[3] = 0;

        console.log(`[Relay] Host resumed in room ${code}`);
        ws.send(JSON.stringify({
          t: 'welcome',
          slot: 0,
          code,
          token: resumeToken,
          roster: room.roster
        }));

        broadcastToRoom(room, JSON.stringify({ t: 'hostback' }), 0);
        setupSocketHandlers(ws, room);
        return;
      }

      // Guest resuming
      for (const [s, tok] of room.tokens.entries()) {
        if (tok === resumeToken) {
          const timer = room.guestHoldTimers.get(s);
          if (timer) {
            clearTimeout(timer);
            room.guestHoldTimers.delete(s);
          }
          room.guests.set(s, ws);
          ws.roomCode = code;
          ws.slot = s;
          ws.token = resumeToken;

          const guestRoster = room.roster.find((r) => r[0] === s);
          if (guestRoster) guestRoster[3] = 0;

          console.log(`[Relay] Guest slot ${s} resumed in room ${code}`);
          ws.send(JSON.stringify({
            t: 'welcome',
            slot: s,
            code,
            token: resumeToken,
            roster: room.roster
          }));

          if (room.host && room.host.readyState === 1) {
            room.host.send(JSON.stringify({ t: 'back', slot: s }));
          }
          broadcastToGuests(room, JSON.stringify({ t: 'back', slot: s }), s);
          setupSocketHandlers(ws, room);
          return;
        }
      }
    }

    // Regular Guest Join
    const availableSlots = [1, 2, 3].filter((s) => !room.guests.has(s) && !room.guestHoldTimers.has(s));
    if (availableSlots.length === 0) {
      console.log(`[Relay] Join rejected: Room ${code} is full`);
      ws.send(JSON.stringify({ t: 'err', e: 4403 }));
      ws.close(4403, 'full');
      return;
    }

    // Check duplicate name
    const nameTaken = room.roster.some((r) => r[1].toUpperCase() === name.toUpperCase() && r[3] === 0);
    if (nameTaken) {
      console.log(`[Relay] Join rejected: Name "${name}" already playing in ${code}`);
      ws.send(JSON.stringify({ t: 'err', e: 4409 }));
      ws.close(4409, 'name');
      return;
    }

    const assignedSlot = availableSlots[0];
    const guestToken = genToken();

    room.guests.set(assignedSlot, ws);
    room.tokens.set(assignedSlot, guestToken);
    room.roster.push([assignedSlot, name, skin, 0]);

    ws.roomCode = code;
    ws.slot = assignedSlot;
    ws.token = guestToken;

    console.log(`[Relay] Guest ${name} (${skin}) joined room ${code} at slot ${assignedSlot}`);

    ws.send(JSON.stringify({
      t: 'welcome',
      slot: assignedSlot,
      code,
      token: guestToken,
      roster: room.roster
    }));

    const joinMsg = JSON.stringify({ t: 'join', slot: assignedSlot, name, skin });
    if (room.host && room.host.readyState === 1) {
      room.host.send(joinMsg);
    }
    broadcastToGuests(room, joinMsg, assignedSlot);

    setupSocketHandlers(ws, room);
  });

  function broadcastToRoom(room, msg, excludeSlot = -1) {
    if (room.host && room.host.readyState === 1 && excludeSlot !== 0) {
      room.host.send(msg);
    }
    broadcastToGuests(room, msg, excludeSlot);
  }

  function broadcastToGuests(room, msg, excludeSlot = -1) {
    for (const [slot, gws] of room.guests.entries()) {
      if (slot !== excludeSlot && gws.readyState === 1) {
        gws.send(msg);
      }
    }
  }

  function setupSocketHandlers(ws, room) {
    ws.on('message', (message, isBinary) => {
      if (!isBinary) {
        const text = message.toString();
        if (text === 'ping') {
          if (ws.readyState === 1) ws.send('pong');
          return;
        }

        if (text === 'leave') {
          handleExplicitLeave(ws, room);
          return;
        }
        return;
      }

      routeBinaryMessage(ws, room, message);
    });

    ws.on('close', () => {
      handleSocketClose(ws, room);
    });

    ws.on('error', (err) => {
      console.warn(`[Relay] Socket error in room ${room.code} (slot ${ws.slot}):`, err.message);
    });
  }

  function routeBinaryMessage(ws, room, data) {
    const buf = Buffer.isBuffer(data) ? data : Buffer.from(data);
    const senderSlot = ws.slot;
    let offset = 0;

    const outPerSlot = { 0: [], 1: [], 2: [], 3: [] };

    while (offset + 6 <= buf.length) {
      const mask = buf[offset];
      const len = buf.readUInt32LE(offset + 2);
      const subFrameTotal = 6 + len;
      if (offset + subFrameTotal > buf.length) break;

      const subBuf = Buffer.allocUnsafe(subFrameTotal);
      buf.copy(subBuf, 0, offset, offset + subFrameTotal);
      subBuf[0] = senderSlot;

      for (let k = 0; k <= 3; k++) {
        if (k !== senderSlot && (mask & (1 << k))) {
          outPerSlot[k].push(subBuf);
        }
      }

      offset += subFrameTotal;
    }

    if (outPerSlot[0].length && room.host && room.host.readyState === 1) {
      room.host.send(Buffer.concat(outPerSlot[0]));
    }

    for (const [slot, gws] of room.guests.entries()) {
      if (outPerSlot[slot] && outPerSlot[slot].length && gws.readyState === 1) {
        gws.send(Buffer.concat(outPerSlot[slot]));
      }
    }
  }

  function handleExplicitLeave(ws, room) {
    if (ws.slot === 0) {
      console.log(`[Relay] Host left room ${room.code}. Closing room.`);
      broadcastToGuests(room, JSON.stringify({ t: 'hostgone' }));
      for (const [, gws] of room.guests.entries()) {
        try { gws.close(4000, 'hostgone'); } catch (e) {}
      }
      rooms.delete(room.code);
    } else {
      console.log(`[Relay] Guest slot ${ws.slot} left room ${room.code}`);
      room.guests.delete(ws.slot);
      room.tokens.delete(ws.slot);
      room.roster = room.roster.filter((r) => r[0] !== ws.slot);
      const leaveMsg = JSON.stringify({ t: 'leave', slot: ws.slot });
      if (room.host && room.host.readyState === 1) {
        room.host.send(leaveMsg);
      }
      broadcastToGuests(room, leaveMsg, ws.slot);
    }
  }

  function handleSocketClose(ws, room) {
    if (!rooms.has(room.code)) return;

    if (ws.slot === 0) {
      if (room.host === ws) {
        console.log(`[Relay] Host disconnected from ${room.code}. Holding slot for 90s.`);
        const hostRoster = room.roster.find((r) => r[0] === 0);
        if (hostRoster) hostRoster[3] = 1;
        broadcastToGuests(room, JSON.stringify({ t: 'hostwait' }));

        if (room.hostHoldTimer) clearTimeout(room.hostHoldTimer);
        room.hostHoldTimer = setTimeout(() => {
          if (room.host === ws) {
            console.log(`[Relay] Host hold expired for ${room.code}. Destroying room.`);
            broadcastToGuests(room, JSON.stringify({ t: 'hostgone' }));
            for (const [, gws] of room.guests.entries()) {
              try { gws.close(4000, 'hostgone'); } catch (e) {}
            }
            rooms.delete(room.code);
          }
        }, 90000);
      }
    } else {
      if (room.guests.get(ws.slot) === ws) {
        console.log(`[Relay] Guest slot ${ws.slot} disconnected from ${room.code}. Holding slot for 30s.`);
        const guestRoster = room.roster.find((r) => r[0] === ws.slot);
        if (guestRoster) guestRoster[3] = 1;
        if (room.host && room.host.readyState === 1) {
          room.host.send(JSON.stringify({ t: 'lost', slot: ws.slot }));
        }

        const slot = ws.slot;
        const timer = setTimeout(() => {
          if (room.guests.get(slot) === ws) {
            console.log(`[Relay] Guest slot ${slot} hold expired in ${room.code}`);
            room.guests.delete(slot);
            room.tokens.delete(slot);
            room.guestHoldTimers.delete(slot);
            room.roster = room.roster.filter((r) => r[0] !== slot);
            const leaveMsg = JSON.stringify({ t: 'leave', slot });
            if (room.host && room.host.readyState === 1) {
              room.host.send(leaveMsg);
            }
            broadcastToGuests(room, leaveMsg, slot);
          }
        }, 30000);

        room.guestHoldTimers.set(slot, timer);
      }
    }
  }

  return {
    server,
    wss,
    start: () => {
      return new Promise((resolve, reject) => {
        server.on('error', (err) => {
          if (err.code === 'EADDRINUSE') {
            console.log(`[Relay] Port ${port} is already in use (another instance of PokéCRAFT or external relay running). Using existing server.`);
            resolve(false);
          } else {
            console.error('[Relay] Server error:', err);
            reject(err);
          }
        });

        server.listen(port, host, () => {
          console.log(`[Relay] PokéCRAFT Multiplayer Relay listening on ${host}:${port}`);
          console.log(`[Relay] Available IP endpoints: ${getLocalIpAddresses().map(ip => `ws://${ip}:${port}`).join(', ')}`);
          resolve(true);
        });
      });
    },
    stop: () => {
      return new Promise((resolve) => {
        for (const room of rooms.values()) {
          try { room.host?.close(); } catch(e){}
          for (const gws of room.guests.values()) {
            try { gws.close(); } catch(e){}
          }
        }
        rooms.clear();
        wss.close(() => {
          server.close(() => resolve());
        });
      });
    }
  };
}

let activeRelayInstance = null;

function startRelayServer(port = 5901, host = '0.0.0.0') {
  if (activeRelayInstance) return Promise.resolve(activeRelayInstance);
  activeRelayInstance = createRelayServer({ port, host });
  return activeRelayInstance.start().then(() => activeRelayInstance);
}

function stopRelayServer() {
  if (!activeRelayInstance) return Promise.resolve();
  const inst = activeRelayInstance;
  activeRelayInstance = null;
  return inst.stop();
}

module.exports = {
  createRelayServer,
  startRelayServer,
  stopRelayServer,
  getLocalIpAddresses
};

if (require.main === module) {
  const port = process.env.PORT || 5901;
  const host = process.env.HOST || '0.0.0.0';
  startRelayServer(port, host).then(() => {
    console.log('[Relay] Standalone relay server running.');
  });
}
