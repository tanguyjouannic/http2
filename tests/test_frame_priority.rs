use http2::{
    frame::{Frame, FrameHeader},
    header::table::HeaderTable,
};

#[test]
pub fn test_priority_frame() {
    // Test parsing PRIORITY frame.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x05, // Length = 31
        0x02, // Frame Type = PRIORITY
        0x00, // Flags = None
        0x00, 0x00, 0x00, 0x03, // Stream Identifier = 3
        0x00, 0x00, 0x00, 0x05, // Stream Dependency = 5
        0x03, // Weight = 3
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
