use http2::{frame::Frame, header::table::HeaderTable};
use http2::frame::data::DataFrame;

#[test]
pub fn test_data_frame_serialize() {
    let data_frame: DataFrame = DataFrame::new(1, true, b"Hello, World!".to_vec());
    let data_frame_bytes = data_frame.serialize(None);

    assert_eq!(data_frame_bytes, vec![
        0x00, 0x00, 0x0d,       // Length = 13
        0x00,                   // Frame Type = DATA
        0x01,                   // Flags = EndStream
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x48, 0x65, 0x6c, 0x6c, 
        0x6f, 0x2c, 0x20, 0x57, 
        0x6f, 0x72, 0x6c, 0x64,
        0x21,                   // Payload  = "Hello, World!"
    ]);

    // Test parsing DATA without padding.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0d, // Length = 13
        0x00, // Frame Type = DATA
        0x01, // Flags = EndStream
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x2c, 0x20, 0x57, 0x6f, 0x72, 0x6c, 0x64,
        0x21, // Payload  = "Hello, World!"
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

#[test]
pub fn test_data_frame_deserialize() {
    let mut data_frame_bytes: Vec<u8> = vec![
        0x00, 0x00, 0x0d,       // Length = 13
        0x00,                   // Frame Type = DATA
        0x01,                   // Flags = EndStream
        0x00, 0x00, 0x00, 0x01, // Stream Identifier = 1
        0x48, 0x65, 0x6c, 0x6c, 
        0x6f, 0x2c, 0x20, 0x57, 
        0x6f, 0x72, 0x6c, 0x64,
        0x21,                   // Payload  = "Hello, World!"
    ];

    let mut header_table = HeaderTable::new(4096);
    let data_frame_deserialized = Frame::deserialize(&mut data_frame_bytes, &mut header_table).unwrap();

    let frame: Frame = Frame::Data(DataFrame::new(1, true, b"Hello, World!".to_vec()));
    assert_eq!(data_frame_deserialized, frame);
}