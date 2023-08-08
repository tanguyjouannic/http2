use http2::header::primitive::HpackInteger;


#[test]
pub fn test_hpack_header_list_huffman() {
    let mut header_table = HeaderTable::new(4096);

    // Example 1 : Request Example with Huffman Coding
    //
    // Header list to encode:
    // :method: GET
    // :scheme: http
    // :path: /
    // :authority: www.example.com
    //
    // Hex dump of encoded data:
    // 8286 8441 8cf1 e3c2 e5f2 3a6b a0ab 90f4 | ...A......:k....
    // ff                                      | .
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
    // 8c                                      |   Literal value (len = 12)
    //                                         |     Huffman encoded:
    // f1e3 c2e5 f23a 6ba0 ab90 f4ff           | .....:k.....
    //                                         |     Decoded:
    //                                         | www.example.com
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
    let mut header_list_encoded: Vec<u8> = vec![
        0x82, 0x86, 0x84, 0x41, 0x8c, 0xf1, 0xe3, 0xc2, 0xe5, 0xf2, 0x3a, 0x6b, 0xa0, 0xab, 0x90,
        0xf4, 0xff,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":method".into(), "GET".into());
    let hf2 = HeaderField::new(":scheme".into(), "http".into());
    let hf3 = HeaderField::new(":path".into(), "/".into());
    let hf4 = HeaderField::new(":authority".into(), "www.example.com".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4]);

    assert_eq!(header_list, expected_header_list);

    // Example 2 : Response Example with Huffman Coding
    //
    // Header list to encode:
    // :method: GET
    // :scheme: http
    // :path: /
    // :authority: www.example.com
    // cache-control: no-cache
    //
    // Hex dump of encoded data:
    // 8286 84be 5886 a8eb 1064 9cbf           | ....X....d..
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
    // 86                                      |   Literal value (len = 6)
    //                                         |     Huffman encoded:
    // a8eb 1064 9cbf                          | ...d..
    //                                         |     Decoded:
    //                                         | no-cache
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
        0x82, 0x86, 0x84, 0xbe, 0x58, 0x86, 0xa8, 0xeb, 0x10, 0x64, 0x9c, 0xbf,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":method".into(), "GET".into());
    let hf2 = HeaderField::new(":scheme".into(), "http".into());
    let hf3 = HeaderField::new(":path".into(), "/".into());
    let hf4 = HeaderField::new(":authority".into(), "www.example.com".into());
    let hf5 = HeaderField::new("cache-control".into(), "no-cache".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4, hf5]);

    assert_eq!(header_list, expected_header_list);

    // Example 3 : Request Example with Huffman Coding
    //
    // Header list to encode:
    // :method: GET
    // :scheme: https
    // :path: /index.html
    // :authority: www.example.com
    // custom-key: custom-value
    //
    // Hex dump of encoded data:
    // 8287 85bf 4088 25a8 49e9 5ba9 7d7f 8925 | ....@.%.I.[.}..%
    // a849 e95b b8e8 b4bf                     | .I.[....
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
    // 88                                      |   Literal name (len = 8)
    //                                         |     Huffman encoded:
    // 25a8 49e9 5ba9 7d7f                     | %.I.[.}.
    //                                         |     Decoded:
    //                                         | custom-key
    // 89                                      |   Literal value (len = 9)
    //                                         |     Huffman encoded:
    // 25a8 49e9 5bb8 e8b4 bf                  | %.I.[....
    //                                         |     Decoded:
    //                                         | custom-value
    //                                         | -> custom-key:
    //                                         |   custom-value
    //
    // Dynamic Table (after decoding):
    // [  1] (s =  54) custom-key: custom-value
    // [  2] (s =  53) cache-control: no-cache
    // [  3] (s =  57) :authority: www.example.com
    //         Table size: 164
    //
    // Decoded header list:
    // :method: GET
    // :scheme: https
    // :path: /index.html
    // :authority: www.example.com
    // custom-key: custom-value
    let mut header_list_encoded: Vec<u8> = vec![
        0x82, 0x87, 0x85, 0xbf, 0x40, 0x88, 0x25, 0xa8, 0x49, 0xe9, 0x5b, 0xa9, 0x7d, 0x7f, 0x89,
        0x25, 0xa8, 0x49, 0xe9, 0x5b, 0xb8, 0xe8, 0xb4, 0xbf,
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

#[test]
pub fn test_hpack_header_list_eviction() {
    // This section shows several consecutive header lists, corresponding to
    // HTTP responses, on the same connection. The HTTP/2 setting parameter
    // SETTINGS_HEADER_TABLE_SIZE is set to the value of 256 octets, causing
    // some evictions to occur.
    let mut header_table = HeaderTable::new(256);

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
    let mut header_list_encoded: Vec<u8> = vec![
        0x48, 0x03, 0x33, 0x30, 0x32, 0x58, 0x07, 0x70, 0x72, 0x69, 0x76, 0x61, 0x74, 0x65, 0x61,
        0x1d, 0x4d, 0x6f, 0x6e, 0x2c, 0x20, 0x32, 0x31, 0x20, 0x4f, 0x63, 0x74, 0x20, 0x32, 0x30,
        0x31, 0x33, 0x20, 0x32, 0x30, 0x3a, 0x31, 0x33, 0x3a, 0x32, 0x31, 0x20, 0x47, 0x4d, 0x54,
        0x6e, 0x17, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x65,
        0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":status".into(), "302".into());
    let hf2 = HeaderField::new("cache-control".into(), "private".into());
    let hf3 = HeaderField::new("date".into(), "Mon, 21 Oct 2013 20:13:21 GMT".into());
    let hf4 = HeaderField::new("location".into(), "https://www.example.com".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4]);

    assert_eq!(header_list, expected_header_list);

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
    let mut header_list_encoded: Vec<u8> = vec![0x48, 0x03, 0x33, 0x30, 0x37, 0xc1, 0xc0, 0xbf];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":status".into(), "307".into());
    let hf2 = HeaderField::new("cache-control".into(), "private".into());
    let hf3 = HeaderField::new("date".into(), "Mon, 21 Oct 2013 20:13:21 GMT".into());
    let hf4 = HeaderField::new("location".into(), "https://www.example.com".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4]);

    assert_eq!(header_list, expected_header_list);

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
    let mut header_list_encoded: Vec<u8> = vec![
        0x88, 0xc1, 0x61, 0x1d, 0x4d, 0x6f, 0x6e, 0x2c, 0x20, 0x32, 0x31, 0x20, 0x4f, 0x63, 0x74,
        0x20, 0x32, 0x30, 0x31, 0x33, 0x20, 0x32, 0x30, 0x3a, 0x31, 0x33, 0x3a, 0x32, 0x32, 0x20,
        0x47, 0x4d, 0x54, 0xc0, 0x5a, 0x04, 0x67, 0x7a, 0x69, 0x70, 0x77, 0x38, 0x66, 0x6f, 0x6f,
        0x3d, 0x41, 0x53, 0x44, 0x4a, 0x4b, 0x48, 0x51, 0x4b, 0x42, 0x5a, 0x58, 0x4f, 0x51, 0x57,
        0x45, 0x4f, 0x50, 0x49, 0x55, 0x41, 0x58, 0x51, 0x57, 0x45, 0x4f, 0x49, 0x55, 0x3b, 0x20,
        0x6d, 0x61, 0x78, 0x2d, 0x61, 0x67, 0x65, 0x3d, 0x33, 0x36, 0x30, 0x30, 0x3b, 0x20, 0x76,
        0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x3d, 0x31,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    // :status: 200
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:22 GMT
    // location: https://www.example.com
    // content-encoding: gzip
    // set-cookie: foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1
    let hf1 = HeaderField::new(":status".into(), "200".into());
    let hf2 = HeaderField::new("cache-control".into(), "private".into());
    let hf3 = HeaderField::new("date".into(), "Mon, 21 Oct 2013 20:13:22 GMT".into());
    let hf4 = HeaderField::new("location".into(), "https://www.example.com".into());
    let hf5 = HeaderField::new("content-encoding".into(), "gzip".into());
    let hf6 = HeaderField::new(
        "set-cookie".into(),
        "foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1".into(),
    );
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4, hf5, hf6]);

    assert_eq!(header_list, expected_header_list);
}

#[test]
pub fn test_hpack_header_list_huffman_eviction() {
    // This section shows the same examples as the previous section but uses
    // Huffman encoding for the literal values.  The HTTP/2 setting
    // parameter SETTINGS_HEADER_TABLE_SIZE is set to the value of 256
    // octets, causing some evictions to occur.  The eviction mechanism uses
    // the length of the decoded literal values, so the same evictions occur
    // as in the previous section.
    let mut header_table = HeaderTable::new(256);

    // Response 1
    //
    // Header list to encode:
    // :status: 302
    // cache-control: private
    // date: Mon, 21 Oct 2013 20:13:21 GMT
    // location: https://www.example.com
    //
    // Hex dump of encoded data:
    // 4882 6402 5885 aec3 771a 4b61 96d0 7abe | H.d.X...w.Ka..z.
    // 9410 54d4 44a8 2005 9504 0b81 66e0 82a6 | ..T.D. .....f...
    // 2d1b ff6e 919d 29ad 1718 63c7 8f0b 97c8 | -..n..)...c.....
    // e9ae 82ae 43d3                          | ....C.
    //
    // Decoding process:
    // 48                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 8)
    //                                         |     :status
    // 82                                      |   Literal value (len = 2)
    //                                         |     Huffman encoded:
    // 6402                                    | d.
    //                                         |     Decoded:
    //                                         | 302
    //                                         | -> :status: 302
    // 58                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 24)
    //                                         |     cache-control
    // 85                                      |   Literal value (len = 5)
    //                                         |     Huffman encoded:
    // aec3 771a 4b                            | ..w.K
    //                                         |     Decoded:
    //                                         | private
    //                                         | -> cache-control: private
    // 61                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 33)
    //                                         |     date
    // 96                                      |   Literal value (len = 22)
    //                                         |     Huffman encoded:
    // d07a be94 1054 d444 a820 0595 040b 8166 | .z...T.D. .....f
    // e082 a62d 1bff                          | ...-..
    //                                         |     Decoded:
    //                                         | Mon, 21 Oct 2013 20:13:21
    //                                         | GMT
    //                                         | -> date: Mon, 21 Oct 2013
    //                                         |   20:13:21 GMT
    // 6e                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 46)
    //                                         |     location
    // 91                                      |   Literal value (len = 17)
    //                                         |     Huffman encoded:
    // 9d29 ad17 1863 c78f 0b97 c8e9 ae82 ae43 | .)...c.........C
    // d3                                      | .
    //                                         |     Decoded:
    //                                         | https://www.example.com
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
    let mut header_list_encoded = vec![
        0x48, 0x82, 0x64, 0x02, 0x58, 0x85, 0xae, 0xc3, 0x77, 0x1a, 0x4b, 0x61, 0x96, 0xd0, 0x7a,
        0xbe, 0x94, 0x10, 0x54, 0xd4, 0x44, 0xa8, 0x20, 0x05, 0x95, 0x04, 0x0b, 0x81, 0x66, 0xe0,
        0x82, 0xa6, 0x2d, 0x1b, 0xff, 0x6e, 0x91, 0x9d, 0x29, 0xad, 0x17, 0x18, 0x63, 0xc7, 0x8f,
        0x0b, 0x97, 0xc8, 0xe9, 0xae, 0x82, 0xae, 0x43, 0xd3,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":status".into(), "302".into());
    let hf2 = HeaderField::new("cache-control".into(), "private".into());
    let hf3 = HeaderField::new("date".into(), "Mon, 21 Oct 2013 20:13:21 GMT".into());
    let hf4 = HeaderField::new("location".into(), "https://www.example.com".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4]);

    assert_eq!(header_list, expected_header_list);

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
    // 4883 640e ffc1 c0bf                     | H.d.....
    //
    // Decoding process:
    // 48                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 8)
    //                                         |     :status
    // 83                                      |   Literal value (len = 3)
    //                                         |     Huffman encoded:
    // 640e ff                                 | d..
    //                                         |     Decoded:
    //                                         | 307
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
    let mut header_list_encoded = vec![0x48, 0x83, 0x64, 0x0e, 0xff, 0xc1, 0xc0, 0xbf];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":status".into(), "307".into());
    let hf2 = HeaderField::new("cache-control".into(), "private".into());
    let hf3 = HeaderField::new("date".into(), "Mon, 21 Oct 2013 20:13:21 GMT".into());
    let hf4 = HeaderField::new("location".into(), "https://www.example.com".into());
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4]);

    assert_eq!(header_list, expected_header_list);

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
    // 88c1 6196 d07a be94 1054 d444 a820 0595 | ..a..z...T.D. ..
    // 040b 8166 e084 a62d 1bff c05a 839b d9ab | ...f...-...Z....
    // 77ad 94e7 821d d7f2 e6c7 b335 dfdf cd5b | w..........5...[
    // 3960 d5af 2708 7f36 72c1 ab27 0fb5 291f | 9`..'..6r..'..).
    // 9587 3160 65c0 03ed 4ee5 b106 3d50 07   | ..1`e...N...=P.
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
    // 96                                      |   Literal value (len = 22)
    //                                         |     Huffman encoded:
    // d07a be94 1054 d444 a820 0595 040b 8166 | .z...T.D. .....f
    // e084 a62d 1bff                          | ...-..
    //                                         |     Decoded:
    //                                         | Mon, 21 Oct 2013 20:13:22
    //                                         | GMT
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
    // 83                                      |   Literal value (len = 3)
    //                                         |     Huffman encoded:
    // 9bd9 ab                                 | ...
    //                                         |     Decoded:
    //                                         | gzip
    //                                         | - evict: date: Mon, 21 Oct
    //                                         |    2013 20:13:21 GMT
    //                                         | -> content-encoding: gzip
    // 77                                      | == Literal indexed ==
    //                                         |   Indexed name (idx = 55)
    //                                         |     set-cookie
    // ad                                      |   Literal value (len = 45)
    //                                         |     Huffman encoded:
    // 94e7 821d d7f2 e6c7 b335 dfdf cd5b 3960 | .........5...[9`
    // d5af 2708 7f36 72c1 ab27 0fb5 291f 9587 | ..'..6r..'..)...
    // 3160 65c0 03ed 4ee5 b106 3d50 07        | 1`e...N...=P.
    //                                         |     Decoded:
    //                                         | foo=ASDJKHQKBZXOQWEOPIUAXQ
    //                                         | WEOIU; max-age=3600; versi
    //                                         | on=1
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
    let mut header_list_encoded: Vec<u8> = vec![
        0x88, 0xc1, 0x61, 0x96, 0xd0, 0x7a, 0xbe, 0x94, 0x10, 0x54, 0xd4, 0x44, 0xa8, 0x20, 0x05,
        0x95, 0x04, 0x0b, 0x81, 0x66, 0xe0, 0x84, 0xa6, 0x2d, 0x1b, 0xff, 0xc0, 0x5a, 0x83, 0x9b,
        0xd9, 0xab, 0x77, 0xad, 0x94, 0xe7, 0x82, 0x1d, 0xd7, 0xf2, 0xe6, 0xc7, 0xb3, 0x35, 0xdf,
        0xdf, 0xcd, 0x5b, 0x39, 0x60, 0xd5, 0xaf, 0x27, 0x08, 0x7f, 0x36, 0x72, 0xc1, 0xab, 0x27,
        0x0f, 0xb5, 0x29, 0x1f, 0x95, 0x87, 0x31, 0x60, 0x65, 0xc0, 0x03, 0xed, 0x4e, 0xe5, 0xb1,
        0x06, 0x3d, 0x50, 0x07,
    ];

    let header_list = HeaderList::decode(&mut header_list_encoded, &mut header_table).unwrap();

    let hf1 = HeaderField::new(":status".into(), "200".into());
    let hf2 = HeaderField::new("cache-control".into(), "private".into());
    let hf3 = HeaderField::new("date".into(), "Mon, 21 Oct 2013 20:13:22 GMT".into());
    let hf4 = HeaderField::new("location".into(), "https://www.example.com".into());
    let hf5 = HeaderField::new("content-encoding".into(), "gzip".into());
    let hf6 = HeaderField::new(
        "set-cookie".into(),
        "foo=ASDJKHQKBZXOQWEOPIUAXQWEOIU; max-age=3600; version=1".into(),
    );
    let expected_header_list = HeaderList::new(vec![hf1, hf2, hf3, hf4, hf5, hf6]);

    assert_eq!(header_list, expected_header_list);
}
