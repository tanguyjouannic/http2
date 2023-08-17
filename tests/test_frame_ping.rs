// use http2::{
//     frame::Frame,
//     header::table::HeaderTable,
// };

// #[test]
// pub fn test_ping_frame() {
//     // Test parsing PING frame with ack.
//     let bytes: Vec<u8> = vec![
//         0x00, 0x00, 0x08, // Length = 8
//         0x06, // Frame Type = PING
//         0x01, // Flags = Ack
//         0x00, 0x00, 0x00, 0x08, // Stream Identifier = 8
//         0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x01, // Opaque Data = 1
//     ];

//     let mut header_table = HeaderTable::new(4096);
//     let frame = Frame::deserialize(bytes, &mut header_table).unwrap();
//     println!("{}", frame);
// }
