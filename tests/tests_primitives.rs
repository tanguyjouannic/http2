use http2::primitive::Http2Integer;

#[test]
pub fn test_encode_integer() {
    let integer = Http2Integer::new(vec![0b00000101, 0b10011101, 0b10110011]); // 368051
    let encoded = integer.encode(5).unwrap();
    assert_eq!(encoded, vec![0b11111, 0b10010100, 0b10111011, 0b10110]); 
}

// 101 10011101 10110011 = 368051 big endian

// 101 10011101 10010100 = 368020 big endian

// 10010100 10011101 101 = 368020 little endian

// 10010100 10111011 10110
