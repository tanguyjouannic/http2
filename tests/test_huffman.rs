use http2::header::huffman::{Tree};
use http2::header::hpack::HpackString;

#[test]
pub fn test_huffman() {
    // Literal Header Field with Indexing
    //
    // The header field representation uses a literal name and a literal
    // value. The header field is added to the dynamic table.
    //
    // Header list to encode:
    // custom-key: custom-header
    //
    // Hex dump of encoded data:
    // 400a 6375 7374 6f6d 2d6b 6579 0d63 7573 | @.custom-key.cus
    // 746f 6d2d 6865 6164 6572                | tom-header
    let mut encoded_data: Vec<u8> = vec![
        0x0a, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x6b, 0x65, 0x79, 0x0d, 0x63, 0x75, 0x73,
        0x74, 0x6f, 0x6d, 0x2d, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    ];

    let custom_key = HpackString::decode(&mut encoded_data).unwrap();
    let custom_header = HpackString::decode(&mut encoded_data).unwrap();
    assert_eq!(custom_key.to_string(), "custom-key".to_string());
    assert_eq!(custom_header.to_string(), "custom-header".to_string());

    let mut encoded_data: Vec<u8> = vec![
        0x8c, 0xf1, 0xe3, 0xc2, 0xe5, 0xf2, 0x3a, 0x6b, 0xa0, 0xab, 0x90, 0xf4, 0xff,
    ];
    
    let custom_key = HpackString::decode(&mut encoded_data).unwrap();
    println!("custom_key: {}", custom_key.to_string());
}
