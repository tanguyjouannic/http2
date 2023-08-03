use http2::header::hpack::{HpackInteger, HeaderTable, HeaderField, HeaderList};


#[test]
pub fn test_hpack_integer() {
    // Example 1: Encoding / Decoding 10 Using a 5-Bit Prefix
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | X | X | X | 0 | 1 | 0 | 1 | 0 |   10 stored on 5 bits
    // +---+---+---+---+---+---+---+---+
    let mask: u8 = 0b10100000;
    let integer = HpackInteger::new(10);

    let mut encoded_integer = integer.encode(5).unwrap();
    assert_eq!(encoded_integer.len(), 1);

    encoded_integer[0] = encoded_integer[0] + mask;
    assert_eq!(0b10101010, encoded_integer[0]);

    encoded_integer.push(0b00010110);

    let decoded_integer = HpackInteger::decode(5, &mut encoded_integer).unwrap();
    assert_eq!(HpackInteger::new(10).value(), decoded_integer.value());
    assert_eq!(encoded_integer.len(), 1);
    assert_eq!(encoded_integer[0], 0b00010110);

    // Example 2: Encoding / Decoding 1337 Using a 5-Bit Prefix
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | X | X | X | 1 | 1 | 1 | 1 | 1 |  Prefix = 31, I = 1306
    // | 1 | 0 | 0 | 1 | 1 | 0 | 1 | 0 |  1306>=128, encode(154), I=1306/128
    // | 0 | 0 | 0 | 0 | 1 | 0 | 1 | 0 |  10<128, encode(10), done
    // +---+---+---+---+---+---+---+---+
    let mask: u8 = 0b10100000;
    let integer = HpackInteger::new(1337);

    let mut encoded_integer = integer.encode(5).unwrap();
    assert_eq!(encoded_integer.len(), 3);

    encoded_integer[0] = encoded_integer[0] + mask;
    assert_eq!(vec![0b10111111, 0b10011010, 0b00001010], encoded_integer);

    encoded_integer.push(0b11111010);

    let decoded_integer = HpackInteger::decode(5, &mut encoded_integer).unwrap();
    assert_eq!(HpackInteger::new(1337).value(), decoded_integer.value());
    assert_eq!(encoded_integer.len(), 1);
    assert_eq!(encoded_integer[0], 0b11111010);

    // Example 3: Encoding / Decoding 42 starting at an Octet Boundary
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 |   42 stored on 8 bits
    // +---+---+---+---+---+---+---+---+
    let integer = HpackInteger::new(42);

    let mut encoded_integer = integer.encode(8).unwrap();
    assert_eq!(vec![0b00101010], encoded_integer);

    encoded_integer.push(0b11111010);

    let decoded_integer = HpackInteger::decode(8, &mut encoded_integer).unwrap();

    assert_eq!(HpackInteger::new(42).value(), decoded_integer.value());
    assert_eq!(encoded_integer.len(), 1);
    assert_eq!(encoded_integer[0], 0b11111010);
}

#[test]
pub fn test_hpack_string() {

}

#[test]
pub fn test_hpack_header_field() {
    // Example 1: Decoding Literal Header Field with Indexing
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
    //
    // Decoding process:
    // 40                                      | == Literal indexed ==
    // 0a                                      |   Literal name (len = 10)
    // 6375 7374 6f6d 2d6b 6579                | custom-key
    // 0d                                      |   Literal value (len = 13)
    // 6375 7374 6f6d 2d68 6561 6465 72        | custom-header
    //                                         | -> custom-key:
    //                                         |   custom-header
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  55) custom-key: custom-header
    //         Table size:  55
    //
    // Decoded header list:
    // custom-key: custom-header
    let mut header_table = HeaderTable::new(4096);

    let mut header_field_encoded: Vec<u8> = vec![
        0x40, 0x0a, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x6b, 0x65, 0x79, 0x0d, 0x63, 0x75, 0x73,
        0x74, 0x6f, 0x6d, 0x2d, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,                
    ];

    let header_field = HeaderField::decode(&mut header_field_encoded, &mut header_table).unwrap();

    assert_eq!(header_field.name(), "custom-key");
    assert_eq!(header_field.value(), "custom-header");
    assert_eq!(header_table.get_dynamic_table_size(), 55);

    // Example 2: Literal Header Field without Indexing
    //
    // The header field representation uses an indexed name and a literal
    // value.  The header field is not added to the dynamic table.
    //
    // Header list to encode:
    // :path: /sample/path
    //
    // Hex dump of encoded data:
    // 040c 2f73 616d 706c 652f 7061 7468      | ../sample/path
    //
    // Decoding process:
    // 04                                      | == Literal not indexed ==
    //                                         |   Indexed name (idx = 4)
    //                                         |     :path
    // 0c                                      |   Literal value (len = 12)
    // 2f73 616d 706c 652f 7061 7468           | /sample/path
    //                                         | -> :path: /sample/path
    //
    // Dynamic table (after decoding): empty.
    //
    // Decoded header list:
    // :path: /sample/path
    let mut header_table = HeaderTable::new(4096);

    let mut header_field_encoded: Vec<u8> = vec![
        0x04, 0x0c, 0x2f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2f, 0x70, 0x61, 0x74, 0x68,
    ];

    let header_field = HeaderField::decode(&mut header_field_encoded, &mut header_table).unwrap();

    assert_eq!(header_field.name(), ":path");
    assert_eq!(header_field.value(), "/sample/path");
    assert_eq!(header_table.get_dynamic_table_size(), 0);
    // Example 3: Literal Header Field Never Indexed
    //
    // The header field representation uses a literal name and a literal
    // value. The header field is not added to the dynamic table and must
    // use the same representation if re-encoded by an intermediary.
    //
    // Header list to encode:
    // password: secret
    //
    // Hex dump of encoded data:
    // 1008 7061 7373 776f 7264 0673 6563 7265 | ..password.secre
    // 74                                      | t
    //
    // Decoding process:
    // 10                                      | == Literal never indexed ==
    // 08                                      |   Literal name (len = 8)
    // 7061 7373 776f 7264                     | password
    // 06                                      |   Literal value (len = 6)
    // 7365 6372 6574                          | secret
    //                                         | -> password: secret
    //
    // Dynamic table (after decoding): empty.
    //
    // Decoded header list:
    // password: secret
    let mut header_table = HeaderTable::new(4096);

    let mut header_field_encoded: Vec<u8> = vec![
        0x10, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x06, 0x73, 0x65, 0x63, 0x72,
        0x65, 0x74,
    ];

    let header_field = HeaderField::decode(&mut header_field_encoded, &mut header_table).unwrap();

    assert_eq!(header_field.name(), "password");
    assert_eq!(header_field.value(), "secret");

    // Example 4 : Indexed Header Field
    //
    // The header field representation uses an indexed header field from the
    // static table.
    //
    // Header list to encode:
    // :method: GET
    //
    // Hex dump of encoded data:
    // 82                                      | .
    //
    // Decoding process:
    // 82                                      | == Indexed - Add ==
    //                                         |   idx = 2
    //                                         | -> :method: GET
    //
    // Dynamic table (after decoding): empty.
    //
    // Decoded header list:
    // :method: GET
    let mut header_table = HeaderTable::new(4096);

    let mut header_field_encoded: Vec<u8> = vec![0x82];

    let header_field = HeaderField::decode(&mut header_field_encoded, &mut header_table).unwrap();

    assert_eq!(header_field.name(), ":method");
    assert_eq!(header_field.value(), "GET");
}

#[test]
pub fn test_hpack_header_list() {
    // Example 1 : Request Examples without Huffman Coding
    //
    // Header list to encode:
    // :method: GET
    // :scheme: http
    // :path: /
    // :authority: www.example.com
    //
    // Hex dump of encoded data:
    // 8286 8441 0f77 7777 2e65 7861 6d70 6c65 | ...A.www.example
    // 2e63 6f6d                               | .com
    //
    // Decoding process:
    // 82                                      | == Indexed - Add ==
    //                                         |   idx = 2
    //                                         | -> :method: GET
    // 86                                      | == Indexed - Add ==
    //                                         |   idx = 6
    //                                         | -> :scheme: http
    // 84                                      | == Indexed - Add ==
    //                                         |   idx = 4
    //                                         | -> :path: /
    // 41                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 1)
    //                                         |     :authority
    // 0f                                      |   Literal value (len = 15)
    // 7777 772e 6578 616d 706c 652e 636f 6d   | www.example.com
    //                                         | -> :authority:
    //                                         |   www.example.com
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  57) :authority: www.example.com
    //         Table size:  57
    //
    // Decoded header list:
    // :method: GET
    // :scheme: http
    // :path: /
    // :authority: www.example.com
    let mut header_table = HeaderTable::new(4096);

    let mut header_list_encoded: Vec<u8> = vec![
        0x82, 0x86, 0x84, 0x41, 0x0f, 0x77, 0x77, 0x77, 0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
        0x65, 0x2e, 0x63, 0x6f, 0x6d,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":method".into(), "GET".into());
    let hf2 = HeaderField::new(":scheme".into(), "http".into());
    let hf3 = HeaderField::new(":path".into(), "/".into());
    let hf4 = HeaderField::new(":authority".into(), "www.example.com".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4]);
    
    assert_eq!(header_list, expected_header_list);

    // Example 2 : Request Examples without Huffman Coding
    //
    // Header list to encode:
    // :method: GET
    // :scheme: http
    // :path: /
    // :authority: www.example.com
    // cache-control: no-cache
    //
    // Hex dump of encoded data:
    // 8286 84be 5808 6e6f 2d63 6163 6865      | ....X.no-cache
    //
    // Decoding process:
    // 82                                      | == Indexed - Add ==
    //                                         |   idx = 2
    //                                         | -> :method: GET
    // 86                                      | == Indexed - Add ==
    //                                         |   idx = 6
    //                                         | -> :scheme: http
    // 84                                      | == Indexed - Add ==
    //                                         |   idx = 4
    //                                         | -> :path: /
    // be                                      | == Indexed - Add ==
    //                                         |   idx = 62
    //                                         | -> :authority:
    //                                         |   www.example.com
    // 58                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 24)
    //                                         |     cache-control
    // 08                                      |   Literal value (len = 8)
    // 6e6f 2d63 6163 6865                     | no-cache
    //                                         | -> cache-control: no-cache
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  53) cache-control: no-cache
    // [  2] (s =  57) :authority: www.example.com
    //         Table size: 110
    //
    // Decoded header list:
    // :method: GET
    // :scheme: http
    // :path: /
    // :authority: www.example.com
    // cache-control: no-cache
    let mut header_list_encoded: Vec<u8> = vec![
        0x82, 0x86, 0x84, 0xbe, 0x58, 0x08, 0x6e, 0x6f, 0x2d, 0x63, 0x61, 0x63, 0x68, 0x65,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":method".into(), "GET".into());
    let hf2 = HeaderField::new(":scheme".into(), "http".into());
    let hf3 = HeaderField::new(":path".into(), "/".into());
    let hf4 = HeaderField::new(":authority".into(), "www.example.com".into());
    let hf5 = HeaderField::new("cache-control".into(), "no-cache".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4, hf5]);

    assert_eq!(header_list, expected_header_list);

    // Example 3 : Response Examples without Huffman Coding
    //
    // Header list to encode:
    // :method: GET
    // :scheme: https
    // :path: /index.html
    // :authority: www.example.com
    // custom-key: custom-value
    //  
    // Hex dump of encoded data:
    // 8287 85bf 400a 6375 7374 6f6d 2d6b 6579 | ....@.custom-key
    // 0c63 7573 746f 6d2d 7661 6c75 65        | .custom-value
    //
    // Decoding process:
    // 82                                      | == Indexed - Add ==
    //                                         |   idx = 2
    //                                         | -> :method: GET
    // 87                                      | == Indexed - Add ==
    //                                         |   idx = 7
    //                                         | -> :scheme: https
    // 85                                      | == Indexed - Add ==
    //                                         |   idx = 5
    //                                         | -> :path: /index.html
    // bf                                      | == Indexed - Add ==
    //                                         |   idx = 63
    //                                         | -> :authority:
    //                                         |   www.example.com
    // 40                                      | == Literal indexed ==
    // 0a                                      |   Literal name (len = 10)
    // 6375 7374 6f6d 2d6b 6579                | custom-key
    // 0c                                      |   Literal value (len = 12)
    // 6375 7374 6f6d 2d76 616c 7565           | custom-value
    //                                         | -> custom-key:
    //                                         |   custom-value
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  54) custom-key: custom-value
    // [  2] (s =  53) cache-control: no-cache
    // [  3] (s =  57) :authority: www.example.com
    //       Table size: 164
    //
    // Decoded header list:
    // :method: GET
    // :scheme: https
    // :path: /index.html
    // :authority: www.example.com
    // custom-key: custom-value
    let mut header_list_encoded: Vec<u8> = vec![
        0x82, 0x87, 0x85, 0xbf, 0x40, 0x0a, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x6b, 0x65,
        0x79, 0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x76, 0x61, 0x6c, 0x75, 0x65,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":method".into(), "GET".into());
    let hf2 = HeaderField::new(":scheme".into(), "https".into());
    let hf3 = HeaderField::new(":path".into(), "/index.html".into());
    let hf4 = HeaderField::new(":authority".into(), "www.example.com".into());
    let hf5 = HeaderField::new("custom-key".into(), "custom-value".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4, hf5]);

    assert_eq!(header_list, expected_header_list);
}