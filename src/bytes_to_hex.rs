pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        hex.push(std::char::from_digit((b >> 4) as u32, 16).unwrap());
        hex.push(std::char::from_digit((b & 0xF) as u32, 16).unwrap());
    }
    hex
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_hex_basic() {
        assert_eq!(bytes_to_hex(b"Man"), "4d616e");
        assert_eq!(bytes_to_hex(b"Hello"), "48656c6c6f");
    }

    #[test]
    fn test_bytes_to_hex_empty() {
        assert_eq!(bytes_to_hex(b""), "");
    }

    #[test]
    fn test_bytes_to_hex_symbols() {
        assert_eq!(bytes_to_hex(b"!\"#$%"), "2122232425");
    }
}