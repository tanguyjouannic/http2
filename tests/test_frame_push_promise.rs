use http2::{
    frame::{Frame, FrameHeader},
    header::table::HeaderTable,
};

#[test]
pub fn test_push_promise_frame() {
    // Test parsing PUSH_PROMISE with padding and end_headers.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x1d, // Length = 31
        0x05, // Frame Type = PUSH_PROMISE
        0x0c, // Flags = [Padded, End_Headers]
        0x00, 0x00, 0x00, 0x03, // Stream Identifier = 3
        0x05, // Pad Length = 5
        0x00, 0x00, 0x00, 0x07, // Promised Stream ID = 7
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
    let frame = Frame::deserialize(frame_header, bytes, &mut header_table).unwrap();

    println!("{}", frame);
}