use http2::primitive::Http2Integer;

#[test]
pub fn test_encode_integer() {
    let integer = Http2Integer::new(vec![0b00000101, 0b10011101, 0b10110011]);
    let encoded = integer.encode(5).unwrap();
    for byte in encoded {
        print!("{:b} ", byte);
    }
}
