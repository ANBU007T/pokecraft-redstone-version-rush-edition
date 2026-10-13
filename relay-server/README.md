# 🌐 PokéCRAFT Global Multiplayer Cloud Relay

This lightweight relay server allows PokéCRAFT players anywhere in the world to connect and play co-op together using **5-letter room codes** (no port forwarding or VPN required).

---

## 🚀 Free 1-Click Deployment Options

### Option A: Render.com (Recommended - 100% Free)
1. Fork or push this repository to your GitHub.
2. Sign in to [Render.com](https://render.com) (free).
3. Click **New +** ➔ **Web Service**.
4. Select your PokéCRAFT repository.
5. Configure:
   - **Root Directory**: `relay-server`
   - **Environment**: `Node`
   - **Build Command**: `npm install`
   - **Start Command**: `node server.js`
   - **Plan**: Free
6. Click **Deploy Web Service**!
7. Render gives you an address like:
   `https://pokecraft-relay.onrender.com`
8. In PokéCRAFT PC / Mobile:
   When clicking **JOIN**, set Host/Server to:
   `wss://pokecraft-relay.onrender.com`
   (Or save it once in game settings!)

---

### Option B: Run Locally on your PC or Home Server
```bash
cd relay-server
npm install
npm start
```
The server will listen on port `5901`. Friends on your local Wi-Fi or Radmin VPN can join using your computer's IP address (e.g. `192.168.1.14:5901` or `26.x.x.x:5901`).
