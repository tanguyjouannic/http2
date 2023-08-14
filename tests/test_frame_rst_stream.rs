use http2::{
    frame::{Frame, FrameHeader},
    header::table::HeaderTable,
};

#[test]
pub fn test_rst_stream_frame() {
    // Test parsing RST_STREAM frame.
    let mut bytes: Vec<u8> = vec![
        0x00, 0x00, 0x04, // Length = 4
        0x03, // Frame Type = RST_STREAM
        0x00, // Flags = None
        0x00, 0x00, 0x00, 0x03, // Stream Identifier = 3
        0x00, 0x00, 0x00, 0x05, // Error code = 5
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
