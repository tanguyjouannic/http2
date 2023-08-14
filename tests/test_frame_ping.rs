use http2::{
    frame::{Frame, FrameHeader},
    header::table::HeaderTable,
};

#[test]
pub fn test_ping_frame() {
    // Test parsing PING frame with ack.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x05, // Length = 5
        0x06, // Frame Type = PING
        0x01, // Flags = Ack
        0x00, 0x00, 0x00, 0x08, // Stream Identifier = 8
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01, // Opaque Data = 1
    ];

    // Create a header table.
    let mut header_table = HeaderTable::new(4096);

    // Retrieve the frame header.
    let frame_header: FrameHeader = bytes[0..9].try_into().unwrap();
    bytes = bytes[9..].to_vec();

    // Deserialize the frame.
    let frame = Frame::deserialize(&frame_header, bytes, &mut header_table).unwrap();

    println!("{}", frame);
}
