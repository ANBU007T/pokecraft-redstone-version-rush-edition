pub mod proto;

use bevy::prelude::*;
use std::collections::HashMap;
pub use proto::*;

#[derive(Debug, Clone)]
pub struct RemotePeer {
    pub slot: usize,
    pub name: String,
    pub skin: String,
    pub tx: i32,
    pub ty: i32,
    pub dir: String,
    pub anim: usize,
    pub latency_ms: u64,
}

#[derive(Resource, Default, Debug)]
pub struct MultiplayerState {
    pub connected: bool,
    pub is_host: bool,
    pub room_code: Option<String>,
    pub local_slot: usize,
    pub peers: HashMap<usize, RemotePeer>,
    pub chat_history: Vec<(String, String)>,
}

impl MultiplayerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn host_room(&mut self, code: impl Into<String>) {
        self.connected = true;
        self.is_host = true;
        self.room_code = Some(code.into());
        self.local_slot = 0;
        self.peers.clear();
    }

    pub fn join_room(&mut self, code: impl Into<String>, slot: usize) {
        self.connected = true;
        self.is_host = false;
        self.room_code = Some(code.into());
        self.local_slot = slot;
        self.peers.clear();
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
        self.is_host = false;
        self.room_code = None;
        self.peers.clear();
    }

    pub fn add_peer(&mut self, slot: usize, name: String, skin: String) {
        self.peers.insert(
            slot,
            RemotePeer {
                slot,
                name,
                skin,
                tx: 0,
                ty: 0,
                dir: "down".to_string(),
                anim: 0,
                latency_ms: 0,
            },
        );
    }

    pub fn update_peer_position(&mut self, slot: usize, tx: i32, ty: i32, dir: String, anim: usize) {
        if let Some(peer) = self.peers.get_mut(&slot) {
            peer.tx = tx;
            peer.ty = ty;
            peer.dir = dir;
            peer.anim = anim;
        }
    }

    pub fn add_chat(&mut self, sender: String, text: String) {
        self.chat_history.push((sender, text));
    }
}
