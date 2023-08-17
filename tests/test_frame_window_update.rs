// use http2::{
//     frame::Frame,
//     header::table::HeaderTable,
// };

// #[test]
// pub fn test_window_update_frame() {
//     // Test parsing WINDOW_UPDATE frame.
//     let bytes: Vec<u8> = vec![
//         0x00, 0x00, 0x04, // Length = 5
//         0x08, // Frame Type = PING
//         0x00, // Flags = Ack
//         0x00, 0x00, 0x00, 0x04, // Stream Identifier = 4
//         0x00, 0x00, 0x00, 0xff, // Window Size Increment = 255
//     ];

//     let mut header_table = HeaderTable::new(4096);
//     let frame = Frame::deserialize(bytes, &mut header_table).unwrap();
//     println!("{}", frame);
// }
