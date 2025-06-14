pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
  let mut bytes = Vec::with_capacity(hex.len() / 2);
  let chars: Vec<char> = hex.chars().collect();
  let len = chars.len();

  let mut c = 0;
  while c + 1 < len {
    let hex_pair = format!("{}{}", chars[c], chars[c + 1]);
    if let Ok(byte) = u8::from_str_radix(&hex_pair, 16) {
      bytes.push(byte);
    }
    c += 2;
  }

  bytes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hex_to_bytes_basic() {
    assert_eq!(hex_to_bytes("4d616e"), b"Man");
    assert_eq!(hex_to_bytes("48656c6c6f"), b"Hello");
  }

  #[test]
  fn test_hex_to_bytes_odd_length() {
    // 忽略最后一个不完整的字符
    assert_eq!(hex_to_bytes("4d61f"), vec![0x4d, 0x61]);
  }

  #[test]
  fn test_hex_to_bytes_invalid() {
    // 非法字符被忽略
    assert_eq!(hex_to_bytes("zz4d61"), vec![0x4d, 0x61]);
  }

  #[test]
  fn test_hex_to_bytes_empty() {
    assert_eq!(hex_to_bytes(""), Vec::<u8>::new());
  }
}
