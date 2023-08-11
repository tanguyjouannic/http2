use http2::frame::Frame;

#[test]
pub fn test_data_frame() {
    // Test parsing Data without padding.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0d, // Length = 13
        0x00, // Frame Type = Data
        0x01, // Flags = EndStream
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x48, 0x65, 0x6c, 0x6c, // Payload  = "Hello, World!"
        0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21,
    ];

    let frame = Frame::try_from(&mut bytes).unwrap();

    println!("{}", frame);

    // Test parsing Data with padding.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0d, // Length = 13
        0x00, // Frame Type = Data
        0x09, // Flags = EndStream + Padded
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x02, // Padding Length = 2
        0x48, 0x65, 0x6c, 0x6c, // Payload  = "Hello, World!"
        0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64, 0x21, // Padding = "d!"
    ];

    let frame = Frame::try_from(&mut bytes).unwrap();

    println!("{}", frame);
}
