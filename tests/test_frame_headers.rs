use http2::{
    frame::{Frame, FrameHeader},
    header::table::HeaderTable,
};

#[test]
pub fn test_headers_frame() {
    // Test parsing HEADERS with padding and priority.
    let mut bytes: Vec<u8> = vec![
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

    // Create a header table.
    let mut header_table = HeaderTable::new(4096);

    // Retrieve the frame header.
    let frame_header: FrameHeader = bytes[0..9].try_into().unwrap();
    bytes = bytes[9..].to_vec();

    // Deserialize the frame.
    let frame = Frame::deserialize(&frame_header, bytes, &mut header_table).unwrap();

    println!("{}", frame);
}
