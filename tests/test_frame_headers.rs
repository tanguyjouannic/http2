use http2::{
    frame::Frame,
    header::table::HeaderTable,
};

#[test]
pub fn test_headers_frame() {
    // Test parsing HEADERS with padding and priority.
    let bytes: Vec<u8> = vec![
        0x00, 0x00, 0x1f, // Length = 31
        0x01, // Frame Type = HEADERS
        0x28, // Flags = [Priority, Padded]
        0x00, 0x00, 0x00, 0x03, // Stream Identifier = 3
        0x05, // Pad Length = 5
        0x00, 0x00, 0x00, 0x05, // Stream Identifier = 5
        0x03, // Weight = 3
        0x82, 0x86, 0x84, 0x41, 0x0f, 0x77, 0x77, 0x77, 0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
        0x65, 0x2e, 0x63, 0x6f, 0x6d,
        // Payload =
        // :method: GET
        // :scheme: http
        // :path: /
        // :authority: www.example.com
        0x01, 0x02, 0x03, 0x04, 0x05, // Padding
    ];

    let mut header_table = HeaderTable::new(4096);
    let frame = Frame::deserialize(bytes, &mut header_table).unwrap();
    println!("{}", frame);
}
