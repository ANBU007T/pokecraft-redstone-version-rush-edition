use pokecraft_bevy::net::proto::{
    NetMessage, encode_frame, decode_frame, validate_room_code, generate_room_code, PROTO_VERSION,
};
use pokecraft_bevy::net::MultiplayerState;

#[test]
fn test_multiplayer_binary_wire_framing() {
    assert_eq!(PROTO_VERSION, 1);

    // 1. Hello message
    let hello = NetMessage::Hello {
        proto_version: 1,
        name: "Steve".to_string(),
        skin: "steve".to_string(),
    };
    let frame_hello = encode_frame(0b00000001, 0, &hello).expect("Frame encode should succeed");
    assert!(frame_hello.len() >= 6);
    assert_eq!(frame_hello[0], 0b00000001); // mask
    assert_eq!(frame_hello[1], 0);          // flags

    let (mask, flags, decoded_hello) = decode_frame(&frame_hello).expect("Frame decode should succeed");
    assert_eq!(mask, 0b00000001);
    assert_eq!(flags, 0);
    assert_eq!(decoded_hello, hello);

    // 2. Move message
    let move_msg = NetMessage::Move {
        tx: 45,
        ty: 120,
        dir: "right".to_string(),
        anim: 1,
    };
    let frame_move = encode_frame(0b00000010, 1, &move_msg).unwrap();
    let (_, _, decoded_move) = decode_frame(&frame_move).unwrap();
    assert_eq!(decoded_move, move_msg);

    // 3. Block edit message
    let edit_msg = NetMessage::BlockEdit {
        x: 46,
        y: 120,
        id: Some("crafting_table".to_string()),
    };
    let frame_edit = encode_frame(0b00000100, 0, &edit_msg).unwrap();
    let (_, _, decoded_edit) = decode_frame(&frame_edit).unwrap();
    assert_eq!(decoded_edit, edit_msg);

    // 4. Chat message
    let chat_msg = NetMessage::Chat {
        sender: "Alex".to_string(),
        text: "Found diamonds!".to_string(),
    };
    let frame_chat = encode_frame(0b00000111, 0, &chat_msg).unwrap();
    let (_, _, decoded_chat) = decode_frame(&frame_chat).unwrap();
    assert_eq!(decoded_chat, chat_msg);
}

#[test]
fn test_room_code_generation_and_validation() {
    // Generate 5-character room code
    let code = generate_room_code(123456789);
    assert_eq!(code.len(), 5);
    assert!(validate_room_code(&code));

    // Valid codes
    assert!(validate_room_code("K7M9P"));
    assert!(validate_room_code("ABCDE"));

    // Invalid codes
    assert!(!validate_room_code("ABC"), "Too short");
    assert!(!validate_room_code("ABCDEF"), "Too long");
    assert!(!validate_room_code("ABCD0"), "Contains forbidden '0'");
    assert!(!validate_room_code("ABCD1"), "Contains forbidden '1'");
    assert!(!validate_room_code("ABCDI"), "Contains forbidden 'I'");
    assert!(!validate_room_code("ABCDO"), "Contains forbidden 'O'");
}

#[test]
fn test_multiplayer_state_and_peer_tracking() {
    let mut net = MultiplayerState::new();

    assert!(!net.connected);
    assert!(!net.is_host);

    // Host room "K7M9P"
    net.host_room("K7M9P");
    assert!(net.connected);
    assert!(net.is_host);
    assert_eq!(net.room_code.as_deref(), Some("K7M9P"));
    assert_eq!(net.local_slot, 0);

    // Add remote peer Alex at slot 1
    net.add_peer(1, "Alex".to_string(), "alex".to_string());
    assert_eq!(net.peers.len(), 1);
    assert!(net.peers.contains_key(&1));
    assert_eq!(net.peers[&1].name, "Alex");

    // Update peer position
    net.update_peer_position(1, 50, 60, "up".to_string(), 2);
    assert_eq!(net.peers[&1].tx, 50);
    assert_eq!(net.peers[&1].ty, 60);
    assert_eq!(net.peers[&1].dir, "up");
    assert_eq!(net.peers[&1].anim, 2);

    // Chat history
    net.add_chat("Alex".to_string(), "Let's fight Ender Dragon!".to_string());
    assert_eq!(net.chat_history.len(), 1);
    assert_eq!(net.chat_history[0].0, "Alex");
    assert_eq!(net.chat_history[0].1, "Let's fight Ender Dragon!");

    // Disconnect
    net.disconnect();
    assert!(!net.connected);
    assert!(net.peers.is_empty());
}
