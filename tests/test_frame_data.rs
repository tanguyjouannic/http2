use http2::{frame::Frame, header::table::HeaderTable};

#[test]
pub fn test_data_frame() {
    // Test parsing DATA without padding.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0d, // Length = 13
        0x00, // Frame Type = DATA
        0x01, // Flags = EndStream
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 
        0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, // Payload  = "Hello, World!"
        0x54, 0x4c, 0x65, 0x73, 0x74, 0xff, // Random bytes
    ];

    let mut header_table = HeaderTable::new(4096);
    let frame = Frame::deserialize(&mut bytes, &mut header_table).unwrap();
    println!("{}", frame);

    // Test parsing DATA with padding.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0e, // Length = 14
        0x00, // Frame Type = Data
        0x09, // Flags = [EndStream, Padded]
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x02, // Pad Length = 2
        0x48, 0x65, 0x6c, 0x6c, // Payload  = "Hello, World!"
        0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21,
    ];

    let mut header_table = HeaderTable::new(4096);
    let frame = Frame::deserialize(&mut bytes, &mut header_table).unwrap();
    println!("{}", frame);
}
