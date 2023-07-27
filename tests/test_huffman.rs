use http2::header::huffman::{Tree};

#[test]
pub fn test_huffman() {
    // let path: Path = Path::from(0xffffea);
    // println!("{:?}", path.directions);
    let tree: Tree = Tree::new().unwrap();

    // Literal Header Field with Indexing
    //
    // The header field representation uses a literal name and a literal
    // value.  The header field is added to the dynamic table.
    //
    // Header list to encode:
    //
    // custom-key: custom-header
    //
    // Hex dump of encoded data:
    //
    // 400a 6375 7374 6f6d 2d6b 6579 0d63 7573 | @.custom-key.cus
    // 746f 6d2d 6865 6164 6572                | tom-header
    let mut encoded_data: Vec<u8> = vec![
        0x40, 0x0a, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x6b, 0x65, 0x79, 0x0d, 0x63, 0x75,
        0x73, 0x74, 0x6f, 0x6d, 0x2d, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    ];

    let s = tree.decode(&mut encoded_data).unwrap();
    println!("{:?}", s);
}
