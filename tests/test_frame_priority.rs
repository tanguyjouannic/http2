use http2::{
    frame::Frame,
    header::table::HeaderTable,
};

#[test]
pub fn test_priority_frame() {
    // Test parsing PRIORITY frame.
    let bytes: Vec<u8> = vec![
        0x00, 0x00, 0x05, // Length = 5
        0x02, // Frame Type = PRIORITY
        0x00, // Flags = None
        0x00, 0x00, 0x00, 0x03, // Stream Identifier = 3
        0x00, 0x00, 0x00, 0x05, // Stream Dependency = 5
        0x03, // Weight = 3
    ];

    let mut header_table = HeaderTable::new(4096);
    let frame = Frame::deserialize(bytes, &mut header_table).unwrap();
    println!("{}", frame);
}
