// use http2::{
//     frame::Frame,
//     header::table::HeaderTable,
// };

// #[test]
// pub fn test_continuation_frame() {
//     // Test parsing CONTINUATION frame with end headers.
//     let bytes: Vec<u8> = vec![
//         0x00, 0x00, 0x14, // Length = 20
//         0x09, // Frame Type = CONTINUATION
//         0x04, // Flags = Ack
//         0x00, 0x00, 0x00, 0x08, // Stream Identifier = 8
//         0x82, 0x86, 0x84, 0x41, 0x0f, 0x77, 0x77, 0x77, 0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
//         0x65, 0x2e, 0x63, 0x6f, 0x6d,
//         // Payload =
//         // :method: GET
//         // :scheme: http
//         // :path: /
//         // :authority: www.example.com
//     ];

//     let mut header_table = HeaderTable::new(4096);
//     let frame = Frame::deserialize(bytes, &mut header_table).unwrap();
//     println!("{}", frame);
// }
