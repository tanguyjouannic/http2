use http2::header::field::{HeaderField, HeaderName, HeaderValue};
use http2::header::representation::HeaderRepresentation;
use http2::header::table::HeaderTable;

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

    let header_name = HeaderName::from("custom-key".to_string());
    let header_value = HeaderValue::from("custom-header".to_string());
    let header_field = HeaderField::new(header_name, header_value);

    let header_representation = header_field.into_representation(&mut header_table);

    let mut bytes = header_representation.encode(false, false);

    assert!(
        bytes
            == vec![
                0x40, 0x0a, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x6b, 0x65, 0x79, 0x0d, 0x63,
                0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72
            ]
    );

    let mut header_table = HeaderTable::new(4096);
    let header_representation = HeaderRepresentation::decode(&mut bytes).unwrap();
    let header_field =
        HeaderField::from_representation(header_representation, &mut header_table).unwrap();

    assert_eq!(
        header_field.clone().unwrap().name(),
        HeaderName::from("custom-key".to_string())
    );
    assert_eq!(
        header_field.clone().unwrap().value(),
        HeaderValue::from("custom-header".to_string())
    );
    assert_eq!(header_table.get_dynamic_table_size(), 55);

    // Example 2: Literal Header Field without Indexing
    //
    // The header field representation uses an indexed name and a literal
    // value. The header field is not added to the dynamic table.
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

    let header_name = HeaderName::from(":path".to_string());
    let header_value = HeaderValue::from("/sample/path".to_string());
    let header_field = HeaderField::new(header_name, header_value);

    let header_representation =
        header_field.into_representation_without_indexing(&mut header_table);

    let mut bytes = header_representation.encode(false, false);

    assert!(
        bytes
            == vec![
                0x04, 0x0c, 0x2f, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2f, 0x70, 0x61, 0x74, 0x68
            ]
    );

    let mut header_table = HeaderTable::new(4096);
    let header_representation = HeaderRepresentation::decode(&mut bytes).unwrap();
    let header_field =
        HeaderField::from_representation(header_representation, &mut header_table).unwrap();

    assert_eq!(
        header_field.clone().unwrap().name(),
        HeaderName::from(":path".to_string())
    );
    assert_eq!(
        header_field.clone().unwrap().value(),
        HeaderValue::from("/sample/path".to_string())
    );
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

    let header_name = HeaderName::from("password".to_string());
    let header_value = HeaderValue::from("secret".to_string());
    let header_field = HeaderField::new(header_name, header_value);

    let header_representation = header_field.into_representation_never_index(&mut header_table);

    let mut bytes = header_representation.encode(false, false);

    assert!(
        bytes
            == vec![
                0x10, 0x08, 0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x06, 0x73, 0x65, 0x63,
                0x72, 0x65, 0x74
            ]
    );

    let mut header_table = HeaderTable::new(4096);

    let header_representation = HeaderRepresentation::decode(&mut bytes).unwrap();
    let header_field =
        HeaderField::from_representation(header_representation, &mut header_table).unwrap();

    assert_eq!(
        header_field.clone().unwrap().name(),
        HeaderName::from("password".to_string())
    );
    assert_eq!(
        header_field.clone().unwrap().value(),
        HeaderValue::from("secret".to_string())
    );
    assert_eq!(header_table.get_dynamic_table_size(), 0);

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

    let header_name = HeaderName::from(":method".to_string());
    let header_value = HeaderValue::from("GET".to_string());
    let header_field = HeaderField::new(header_name, header_value);

    let header_representation = header_field.into_representation(&mut header_table);

    let mut bytes = header_representation.encode(false, false);

    assert!(bytes == vec![0x82]);

    let mut header_table = HeaderTable::new(4096);

    let header_representation = HeaderRepresentation::decode(&mut bytes).unwrap();
    let header_field =
        HeaderField::from_representation(header_representation, &mut header_table).unwrap();

    assert_eq!(
        header_field.clone().unwrap().name(),
        HeaderName::from(":method".to_string())
    );
    assert_eq!(
        header_field.clone().unwrap().value(),
        HeaderValue::from("GET".to_string())
    );
    assert_eq!(header_table.get_dynamic_table_size(), 0);
}
