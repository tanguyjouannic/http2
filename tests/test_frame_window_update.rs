use http2::{
    frame::{Frame, FrameHeader},
    header::table::HeaderTable,
};

#[test]
pub fn test_window_update_frame() {
    // Test parsing WINDOW_UPDATE frame.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x04, // Length = 5
        0x08, // Frame Type = PING
        0x00, // Flags = Ack
        0x00, 0x00, 0x00, 0x04, // Stream Identifier = 4
        0x00, 0x00, 0x00, 0xff, // Window Size Increment = 255
    ];

    // Create a header table.
    let mut header_table = HeaderTable::new(4096);

    // Retrieve the frame header.
    let frame_header: FrameHeader = bytes[0..9].try_into().unwrap();
    bytes = bytes[9..].to_vec();

    // Deserialize the frame.
    let frame = Frame::deserialize(frame_header, bytes, &mut header_table).unwrap();

    println!("{}", frame);
}
