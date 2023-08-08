use http2::header::list::HeaderList;
use http2::header::table::HeaderTable;
use http2::header::field::{HeaderField, HeaderName, HeaderValue};


#[test]
pub fn test_header_list() {
    let mut header_table_sender = HeaderTable::new(4096);
    let mut header_table_receiver = HeaderTable::new(4096);

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
    let header_field_1: HeaderField = HeaderField::new(
        HeaderName::from(":method".to_string()),
        HeaderValue::from("GET".to_string())
    );
    let header_field_2: HeaderField = HeaderField::new(
        HeaderName::from(":scheme".to_string()),
        HeaderValue::from("http".to_string())
    );
    let header_field_3: HeaderField = HeaderField::new(
        HeaderName::from(":path".to_string()),
        HeaderValue::from("/".to_string())
    );
    let header_field_4: HeaderField = HeaderField::new(
        HeaderName::from(":authority".to_string()),
        HeaderValue::from("www.example.com".to_string())
    );
    let header_list = HeaderList::from(vec![
        header_field_1,
        header_field_2,
        header_field_3,
        header_field_4
    ]);

    let mut encoded_header_list = header_list.encode(&mut header_table_sender).unwrap();

    assert_eq!(encoded_header_list, vec![
        0x82, 0x86, 0x84, 0x41, 0x0f, 0x77, 0x77, 0x77,
        0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65,
        0x2e, 0x63, 0x6f, 0x6d
    ]);

    assert_eq!(header_table_sender.get_dynamic_table_size(), 57);

    let decoded_header_list = HeaderList::decode(&mut encoded_header_list, &mut header_table_receiver).unwrap();

    assert_eq!(decoded_header_list, header_list);
    assert_eq!(header_table_receiver.get_dynamic_table_size(), 57);

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
    let header_field_1: HeaderField = HeaderField::new(
        HeaderName::from(":method".to_string()),
        HeaderValue::from("GET".to_string())
    );
    let header_field_2: HeaderField = HeaderField::new(
        HeaderName::from(":scheme".to_string()),
        HeaderValue::from("http".to_string())
    );
    let header_field_3: HeaderField = HeaderField::new(
        HeaderName::from(":path".to_string()),
        HeaderValue::from("/".to_string())
    );
    let header_field_4: HeaderField = HeaderField::new(
        HeaderName::from(":authority".to_string()),
        HeaderValue::from("www.example.com".to_string())
    );
    let header_field_5: HeaderField = HeaderField::new(
        HeaderName::from("cache-control".to_string()),
        HeaderValue::from("no-cache".to_string())
    );
    let header_list = HeaderList::from(vec![
        header_field_1,
        header_field_2,
        header_field_3,
        header_field_4,
        header_field_5
    ]);

    let mut encoded_header_list = header_list.encode(&mut header_table_sender).unwrap();

    assert_eq!(encoded_header_list, vec![
        0x82, 0x86, 0x84, 0xbe, 0x58, 0x08, 0x6e, 0x6f,
        0x2d, 0x63, 0x61, 0x63, 0x68, 0x65
    ]);
    assert_eq!(header_table_sender.get_dynamic_table_size(), 110);

    let decoded_header_list = HeaderList::decode(&mut encoded_header_list, &mut header_table_receiver).unwrap();

    assert_eq!(decoded_header_list, header_list);
    assert_eq!(header_table_receiver.get_dynamic_table_size(), 110);

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
    let header_field_1: HeaderField = HeaderField::new(
        HeaderName::from(":method".to_string()),
        HeaderValue::from("GET".to_string())
    );
    let header_field_2: HeaderField = HeaderField::new(
        HeaderName::from(":scheme".to_string()),
        HeaderValue::from("https".to_string())
    );
    let header_field_3: HeaderField = HeaderField::new(
        HeaderName::from(":path".to_string()),
        HeaderValue::from("/index.html".to_string())
    );
    let header_field_4: HeaderField = HeaderField::new(
        HeaderName::from(":authority".to_string()),
        HeaderValue::from("www.example.com".to_string())
    );
    let header_field_5: HeaderField = HeaderField::new(
        HeaderName::from("custom-key".to_string()),
        HeaderValue::from("custom-value".to_string())
    );
    let header_list = HeaderList::from(vec![
        header_field_1,
        header_field_2,
        header_field_3,
        header_field_4,
        header_field_5
    ]);

    let mut encoded_header_list = header_list.encode(&mut header_table_sender).unwrap();

    assert_eq!(encoded_header_list, vec![
        0x82, 0x87, 0x85, 0xbf, 0x40, 0x0a, 0x63, 0x75,
        0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x6b, 0x65, 0x79,
        0x0c, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d,
        0x76, 0x61, 0x6c, 0x75, 0x65
    ]);
    assert_eq!(header_table_sender.get_dynamic_table_size(), 164);

    let decoded_header_list = HeaderList::decode(&mut encoded_header_list, &mut header_table_receiver).unwrap();

    assert_eq!(decoded_header_list, header_list);
    assert_eq!(header_table_receiver.get_dynamic_table_size(), 164);
}

#[test]
pub fn test_header_list_eviction() {
    // This section shows several consecutive header lists, corresponding to
    // HTTP responses, on the same connection. The HTTP/2 setting parameter
    // SETTINGS_HEADER_TABLE_SIZE is set to the value of 256 octets, causing
    // some evictions to occur.
    let mut header_table_sender = HeaderTable::new(256);
    let mut header_table_receiver = HeaderTable::new(256);

    // Response 1
    //
    // Header list to encode:
    // :status: 302
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:21 GMT
    // location: https://www.example.com
    //
    // Hex dump of encoded data:
    // 4803 3330 3258 0770 7269 7661 7465 611d | H.302X.privatea.
    // 4d6f 6e2c 2032 3120 4f63 7420 3230 3133 | Mon, 21 Oct 2013
    // 2032 303a 3133 3a32 3120 474d 546e 1768 |  20:13:21 GMTn.h
    // 7474 7073 3a2f 2f77 7777 2e65 7861 6d70 | ttps://www.examp
    // 6c65 2e63 6f6d                          | le.com
    //
    // Decoding process:
    // 48                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 8)
    //                                         |     :status
    // 03                                      |   Literal value (len = 3)
    // 3330 32                                 | 302
    //                                         | -> :status: 302
    // 58                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 24)
    //                                         |     cache-control
    // 07                                      |   Literal value (len = 7)
    // 7072 6976 6174 65                       | private
    //                                         | -> cache-control: private
    // 61                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 33)
    //                                         |     date
    // 1d                                      |   Literal value (len = 29)
    // 4d6f 6e2c 2032 3120 4f63 7420 3230 3133 | Mon, 21 Oct 2013
    // 2032 303a 3133 3a32 3120 474d 54        |  20:13:21 GMT
    //                                         | -> date: Mon, 21 Oct 2013
    //                                         |   20:13:21 GMT
    // 6e                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 46)
    //                                         |     location
    // 17                                      |   Literal value (len = 23)
    // 6874 7470 733a 2f2f 7777 772e 6578 616d | https://www.exam
    // 706c 652e 636f 6d                       | ple.com
    //                                         | -> location:
    //                                         |   https://www.example.com
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  63) location: https://www.example.com
    // [  2] (s =  65) date: Mon, 21 Oct 2013 20:13:21 GMT
    // [  3] (s =  52) cache-control: private
    // [  4] (s =  42) :status: 302
    //         Table size: 222
    //
    // Decoded header list:
    // :status: 302
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:21 GMT
    // location: https://www.example.com
    let header_field_1: HeaderField = HeaderField::new(
        HeaderName::from(":status".to_string()),
        HeaderValue::from("302".to_string())
    );
    let header_field_2: HeaderField = HeaderField::new(
        HeaderName::from("cache-control".to_string()),
        HeaderValue::from("private".to_string())
    );
    let header_field_3: HeaderField = HeaderField::new(
        HeaderName::from("date".to_string()),
        HeaderValue::from("Mon, 21 Oct 2013 20:13:21 GMT".to_string())
    );
    let header_field_4: HeaderField = HeaderField::new(
        HeaderName::from("location".to_string()),
        HeaderValue::from("https://www.example.com".to_string())
    );
    let header_list = HeaderList::new(vec![
        header_field_1,
        header_field_2,
        header_field_3,
        header_field_4
    ]);

    let mut encoded_header_list = header_list.encode(&mut header_table_sender).unwrap();
    
    assert_eq!(encoded_header_list, vec![
        0x48, 0x03, 0x33, 0x30, 0x32, 0x58, 0x07, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x61,
        0x1d, 0x4d, 0x6f, 0x6e, 0x2c, 0x20, 0x32, 0x31, 0x20, 0x4f, 0x63, 0x74, 0x20, 0x32, 0x30,
        0x31, 0x33, 0x20, 0x32, 0x30, 0x3a, 0x31, 0x33, 0x3a, 0x32, 0x31, 0x20, 0x47, 0x4d, 0x54,
        0x6e, 0x17, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x65,
        0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d,
    ]);

    assert_eq!(header_table_sender.get_dynamic_table_size(), 222);

    let decoded_header_list = HeaderList::decode(&mut encoded_header_list, &mut header_table_receiver).unwrap();

    assert_eq!(decoded_header_list, header_list);
    assert_eq!(header_table_receiver.get_dynamic_table_size(), 222);

    // Response 2
    //
    // The (":status", "302") header field is evicted from the dynamic table
    // to free space to allow adding the (":status", "307") header field.
    //
    // Header list to encode:
    // :status: 307
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:21 GMT
    // location: https://www.example.com
    //
    // Hex dump of encoded data:
    // 4803 3330 37c1 c0bf                     | H.307...
    //
    // Decoding process:
    // 48                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 8)
    //                                         |     :status
    // 03                                      |   Literal value (len = 3)
    // 3330 37                                 | 307
    //                                         | - evict: :status: 302
    //                                         | -> :status: 307
    // c1                                      | == Indexed - Add ==
    //                                         |   idx = 65
    //                                         | -> cache-control: private
    // c0                                      | == Indexed - Add ==
    //                                         |   idx = 64
    //                                         | -> date: Mon, 21 Oct 2013
    //                                         |   20:13:21 GMT
    // bf                                      | == Indexed - Add ==
    //                                         |   idx = 63
    //                                         | -> location:
    //                                         |   https://www.example.com
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  42) :status: 307
    // [  2] (s =  63) location: https://www.example.com
    // [  3] (s =  65) date: Mon, 21 Oct 2013 20:13:21 GMT
    // [  4] (s =  52) cache-control: private
    //         Table size: 222
    //
    // Decoded header list:
    // :status: 307
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:21 GMT
    // location: https://www.example.com
    let header_field_1: HeaderField = HeaderField::new(
        HeaderName::from(":status".to_string()),
        HeaderValue::from("307".to_string())
    );
    let header_field_2: HeaderField = HeaderField::new(
        HeaderName::from("cache-control".to_string()),
        HeaderValue::from("private".to_string())
    );
    let header_field_3: HeaderField = HeaderField::new(
        HeaderName::from("date".to_string()),
        HeaderValue::from("Mon, 21 Oct 2013 20:13:21 GMT".to_string())
    );
    let header_field_4: HeaderField = HeaderField::new(
        HeaderName::from("location".to_string()),
        HeaderValue::from("https://www.example.com".to_string())
    );
    let header_list = HeaderList::new(vec![
        header_field_1,
        header_field_2,
        header_field_3,
        header_field_4
    ]);

    let mut encoded_header_list = header_list.encode(&mut header_table_sender).unwrap();

    assert_eq!(encoded_header_list, vec![
        0x48, 0x03, 0x33, 0x30, 0x37, 0xc1, 0xc0, 0xbf
    ]);
    assert_eq!(header_table_sender.get_dynamic_table_size(), 222);

    let decoded_header_list = HeaderList::decode(&mut encoded_header_list, &mut header_table_receiver).unwrap();

    assert_eq!(decoded_header_list, header_list);
    assert_eq!(header_table_receiver.get_dynamic_table_size(), 222);

    // Response 3
    //
    // Several header fields are evicted from the dynamic table during the
    // processing of this header list.
    //
    // Header list to encode:
    // :status: 200
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:22 GMT
    // location: https://www.example.com
    // content-encoding: gzip
    // set-cookie: foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1
    //
    // Hex dump of encoded data:
    // 88c1 611d 4d6f 6e2c 2032 3120 4f63 7420 | ..a.Mon, 21 Oct
    // 3230 3133 2032 303a 3133 3a32 3220 474d | 2013 20:13:22 GM
    // 54c0 5a04 677a 6970 7738 666f 6f3d 4153 | T.Z.gzipw8foo=AS
    // 444a 4b48 514b 425a 584f 5157 454f 5049 | DJKHQKBZXOQWEOPI
    // 5541 5851 5745 4f49 553b 206d 6178 2d61 | UAXQWEOIU; max-a
    // 6765 3d33 3630 303b 2076 6572 7369 6f6e | ge=3600; version
    // 3d31                                    | =1
    //
    // Decoding process:
    // 88                                      | == Indexed - Add ==
    //                                         |   idx = 8
    //                                         | -> :status: 200
    // c1                                      | == Indexed - Add ==
    //                                         |   idx = 65
    //                                         | -> cache-control: private
    // 61                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 33)
    //                                         |     date
    // 1d                                      |   Literal value (len = 29)
    // 4d6f 6e2c 2032 3120 4f63 7420 3230 3133 | Mon, 21 Oct 2013
    // 2032 303a 3133 3a32 3220 474d 54        |  20:13:22 GMT
    //                                         | - evict: cache-control:
    //                                         |   private
    //                                         | -> date: Mon, 21 Oct 2013
    //                                         |   20:13:22 GMT
    // c0                                      | == Indexed - Add ==
    //                                         |   idx = 64
    //                                         | -> location:
    //                                         |   https://www.example.com
    // 5a                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 26)
    //                                         |     content-encoding
    // 04                                      |   Literal value (len = 4)
    // 677a 6970                               | gzip
    //                                         | - evict: date: Mon, 21 Oct
    //                                         |    2013 20:13:21 GMT
    //                                         | -> content-encoding: gzip
    // 77                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 55)
    //                                         |     set-cookie
    // 38                                      |   Literal value (len = 56)
    // 666f 6f3d 4153 444a 4b48 514b 425a 584f | foo=ASDJKHQKBZXO
    // 5157 454f 5049 5541 5851 5745 4f49 553b | QWEOPIUAXQWEOIU;
    // 206d 6178 2d61 6765 3d33 3630 303b 2076 |  max-age=3600; v
    // 6572 7369 6f6e 3d31                     | ersion=1
    //                                         | - evict: location:
    //                                         |   https://www.example.com
    //                                         | - evict: :status: 307
    //                                         | -> set-cookie: foo=ASDJKHQ
    //                                         |   KBZXOQWEOPIUAXQWEOIU; ma
    //                                         |   x-age=3600; version=1
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  98) set-cookie: foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU;
    //                     max-age=3600; version=1
    // [  2] (s =  52) content-encoding: gzip
    // [  3] (s =  65) date: Mon, 21 Oct 2013 20:13:22 GMT
    //         Table size: 215
    //
    // Decoded header list:
    // :status: 200
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:22 GMT
    // location: https://www.example.com
    // content-encoding: gzip
    // set-cookie: foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1
    let header_field_1: HeaderField = HeaderField::new(
        HeaderName::from(":status".to_string()),
        HeaderValue::from("200".to_string())
    );
    let header_field_2: HeaderField = HeaderField::new(
        HeaderName::from("cache-control".to_string()),
        HeaderValue::from("private".to_string())
    );
    let header_field_3: HeaderField = HeaderField::new(
        HeaderName::from("date".to_string()),
        HeaderValue::from("Mon, 21 Oct 2013 20:13:22 GMT".to_string())
    );
    let header_field_4: HeaderField = HeaderField::new(
        HeaderName::from("location".to_string()),
        HeaderValue::from("https://www.example.com".to_string())
    );
    let header_field_5: HeaderField = HeaderField::new(
        HeaderName::from("content-encoding".to_string()),
        HeaderValue::from("gzip".to_string())
    );
    let header_field_6: HeaderField = HeaderField::new(
        HeaderName::from("set-cookie".to_string()),
        HeaderValue::from("foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1".to_string())
    );
    let header_list: HeaderList = HeaderList::new(vec![
        header_field_1,
        header_field_2,
        header_field_3,
        header_field_4,
        header_field_5,
        header_field_6
    ]);

    let mut encoded_header_list = header_list.encode(&mut header_table_sender).unwrap();

    assert_eq!(encoded_header_list, vec![
        0x88, 0xc1, 0x61, 0x1d, 0x4d, 0x6f, 0x6e, 0x2c, 0x20, 0x32, 0x31, 0x20, 0x4f, 0x63, 0x74,
        0x20, 0x32, 0x30, 0x31, 0x33, 0x20, 0x32, 0x30, 0x3a, 0x31, 0x33, 0x3a, 0x32, 0x32, 0x20,
        0x47, 0x4d, 0x54, 0xc0, 0x5a, 0x04, 0x67, 0x7a, 0x69, 0x70, 0x77, 0x38, 0x66, 0x6f, 0x6f,
        0x3d, 0x41, 0x53, 0x44, 0x4a, 0x4b, 0x48, 0x51, 0x4b, 0x42, 0x5a, 0x58, 0x4f, 0x51, 0x57,
        0x45, 0x4f, 0x50, 0x49, 0x55, 0x41, 0x58, 0x51, 0x57, 0x45, 0x4f, 0x49, 0x55, 0x3b, 0x20,
        0x6d, 0x61, 0x78, 0x2d, 0x61, 0x67, 0x65, 0x3d, 0x33, 0x36, 0x30, 0x30, 0x3b, 0x20, 0x76,
        0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x3d, 0x31,
    ]);
    assert_eq!(header_table_sender.get_dynamic_table_size(), 215);

    let decoded_header_list = HeaderList::decode(&mut encoded_header_list, &mut header_table_receiver).unwrap();

    assert_eq!(decoded_header_list, header_list);
    assert_eq!(header_table_receiver.get_dynamic_table_size(), 215);
}