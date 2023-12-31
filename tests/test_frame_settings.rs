use http2::{frame::Frame, header::table::HeaderTable};

#[test]
pub fn test_settings_frame() {
    // Test parsing SETTINGS frame.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0c, // Length = 12
        0x04, // Frame Type = SETTINGS
        0x00, // Flags = None
        0x00, 0x00, 0x00, 0x03, // Stream Identifier = 3
        0x00, 0x01, // Parameter Identifier = SETTINGS_HEADER_TABLE_SIZE
        0x00, 0x00, 0x00, 0xff, // Parameter Value = 255
        0x00, 0x02, // Parameter Identifier = SETTINGS_ENABLE_PUSH
        0x00, 0x00, 0x00, 0x00, // Parameter Value = 0
    ];

    let mut header_table = HeaderTable::new(4096);
    let frame = Frame::deserialize(&mut bytes, &mut header_table).unwrap();
    println!("{}", frame);
}
