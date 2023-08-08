use http2::header::primitive::HpackInteger;

#[test]
pub fn test_hpack_integer() {
    // Example 1: Encoding / Decoding 10 Using a 5-Bit Prefix
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | X | X | X | 0 | 1 | 0 | 1 | 0 |   10 stored on 5 bits
    // +---+---+---+---+---+---+---+---+
    let mask: u8 = 0b10100000;
    let integer = HpackInteger::from(10 as u128);

    let mut encoded_integer = integer.encode(5).unwrap();
    assert_eq!(encoded_integer.len(), 1);

    encoded_integer[0] = encoded_integer[0] + mask;
    assert_eq!(0b10101010, encoded_integer[0]);

    encoded_integer.push(0b00010110);

    let decoded_integer = HpackInteger::decode(5, &mut encoded_integer).unwrap();
    assert_eq!(HpackInteger::from(10 as u128), decoded_integer);
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
    let integer = HpackInteger::from(1337 as u128);

    let mut encoded_integer = integer.encode(5).unwrap();
    assert_eq!(encoded_integer.len(), 3);

    encoded_integer[0] = encoded_integer[0] + mask;
    assert_eq!(vec![0b10111111, 0b10011010, 0b00001010], encoded_integer);

    encoded_integer.push(0b11111010);

    let decoded_integer = HpackInteger::decode(5, &mut encoded_integer).unwrap();
    assert_eq!(HpackInteger::from(1337 as u128), decoded_integer);
    assert_eq!(encoded_integer.len(), 1);
    assert_eq!(encoded_integer[0], 0b11111010);

    // Example 3: Encoding / Decoding 42 starting at an Octet Boundary
    //
    //   0   1   2   3   4   5   6   7
    // +---+---+---+---+---+---+---+---+
    // | 0 | 0 | 1 | 0 | 1 | 0 | 1 | 0 |   42 stored on 8 bits
    // +---+---+---+---+---+---+---+---+
    let integer = HpackInteger::from(42 as u128);

    let mut encoded_integer = integer.encode(8).unwrap();
    assert_eq!(vec![0b00101010], encoded_integer);

    encoded_integer.push(0b11111010);

    let decoded_integer = HpackInteger::decode(8, &mut encoded_integer).unwrap();

    assert_eq!(HpackInteger::from(42 as u128), decoded_integer);
    assert_eq!(encoded_integer.len(), 1);
    assert_eq!(encoded_integer[0], 0b11111010);
}

#[test]
pub fn test_hpack_string() {}