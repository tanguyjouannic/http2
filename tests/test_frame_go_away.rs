use http2::{frame::Frame, header::table::HeaderTable};

#[test]
pub fn test_goaway_frame() {
    // Test parsing GOAWAY frame.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x15, // Length = 21
        0x07, // Frame Type = GOAWAY
        0x00, // Flags = None
        0x00, 0x00, 0x00, 0x06, // Stream Identifier = 6
        0x00, 0x00, 0x00, 0x05, // Last Stream Identifier = 5
        0x00, 0x00, 0x00, 0x01, // Error Code = 1
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
        0x21, // Additional Debug Data = "Hello World!"
    ];

    let mut header_table = HeaderTable::new(4096);
    let frame = Frame::deserialize(&mut bytes, &mut header_table).unwrap();
    println!("{}", frame);
}
