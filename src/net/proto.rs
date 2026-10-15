use serde::{Deserialize, Serialize};

pub const PROTO_VERSION: u32 = 1;
pub const ROOM_CODE_CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum NetMessage {
    #[serde(rename = "hello")]
    Hello {
        proto_version: u32,
        name: String,
        skin: String,
    },
    #[serde(rename = "room")]
    Room {
        code: String,
        slot: usize,
    },
    #[serde(rename = "move")]
    Move {
        tx: i32,
        ty: i32,
        dir: String,
        anim: usize,
    },
    #[serde(rename = "block")]
    BlockEdit {
        x: i32,
        y: i32,
        id: Option<String>,
    },
    #[serde(rename = "chat")]
    Chat {
        sender: String,
        text: String,
    },
    #[serde(rename = "ping")]
    Ping {
        timestamp_ms: u64,
    },
    #[serde(rename = "pong")]
    Pong {
        timestamp_ms: u64,
    },
}

/// Encodes a network sub-frame: [u8 mask][u8 flags][u32 len LE][UTF-8 JSON bytes]
pub fn encode_frame(mask: u8, flags: u8, msg: &NetMessage) -> Result<Vec<u8>, serde_json::Error> {
    let json_bytes = serde_json::to_vec(msg)?;
    let len = json_bytes.len() as u32;

    let mut buf = Vec::with_capacity(6 + json_bytes.len());
    buf.push(mask);
    buf.push(flags);
    buf.extend_from_slice(&len.to_le_bytes());
    buf.extend_from_slice(&json_bytes);

    Ok(buf)
}

/// Decodes a network sub-frame from raw binary bytes
pub fn decode_frame(buf: &[u8]) -> Result<(u8, u8, NetMessage), Box<dyn std::error::Error>> {
    if buf.len() < 6 {
        return Err("Buffer too small for frame header".into());
    }

    let mask = buf[0];
    let flags = buf[1];
    let len_bytes: [u8; 4] = [buf[2], buf[3], buf[4], buf[5]];
    let len = u32::from_le_bytes(len_bytes) as usize;

    if buf.len() < 6 + len {
        return Err("Incomplete frame payload".into());
    }

    let msg: NetMessage = serde_json::from_slice(&buf[6..6 + len])?;
    Ok((mask, flags, msg))
}

pub fn validate_room_code(code: &str) -> bool {
    if code.len() != 5 {
        return false;
    }
    code.bytes().all(|b| ROOM_CODE_CHARS.contains(&b))
}

pub fn generate_room_code(seed: u64) -> String {
    let mut code = String::with_capacity(5);
    let mut s = seed;
    for _ in 0..5 {
        let idx = (s % ROOM_CODE_CHARS.len() as u64) as usize;
        code.push(ROOM_CODE_CHARS[idx] as char);
        s = s.wrapping_mul(1103515245).wrapping_add(12345);
    }
    code
}
