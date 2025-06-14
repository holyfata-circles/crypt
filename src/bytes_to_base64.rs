pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let base64map = crate::BASE64_MAP;
    let mut base64 = String::new();
    let len = bytes.len();

    let mut i = 0;
    while i < len {
        let b0 = bytes[i] as u32;
        let b1 = if i + 1 < len { bytes[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < len { bytes[i + 2] as u32 } else { 0 };

        let triplet = (b0 << 16) | (b1 << 8) | b2;

        for j in 0..4 {
            if i * 8 + j * 6 < len * 8 {
                let idx = ((triplet >> (6 * (3 - j))) & 0x3F) as usize;
                base64.push(base64map.chars().nth(idx).unwrap());
            } else {
                base64.push('=');
            }
        }
        i += 3;
    }

    base64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_base64_simple() {
        // "Man" -> "TWFu"
        let input = b"Man";
        let expected = "TWFu";
        assert_eq!(bytes_to_base64(input), expected);
    }

    #[test]
    fn test_bytes_to_base64_with_padding() {
        // "Ma" -> "TWE="
        let input = b"Ma";
        let expected = "TWE=";
        assert_eq!(bytes_to_base64(input), expected);

        // "M" -> "TQ=="
        let input2 = b"M";
        let expected2 = "TQ==";
        assert_eq!(bytes_to_base64(input2), expected2);
    }

    #[test]
    fn test_bytes_to_base64_all_ascii() {
        // "ABCDEFGHIJ" -> "QUJDREVGR0hJSg=="
        let input = b"ABCDEFGHIJ";
        let expected = "QUJDREVGR0hJSg==";
        assert_eq!(bytes_to_base64(input), expected);
    }

    #[test]
    fn test_bytes_to_base64_non_ascii() {
        // "中文" -> "5Lit5paH"
        let input = "中文".as_bytes();
        let expected = "5Lit5paH";
        assert_eq!(bytes_to_base64(input), expected);
    }

    #[test]
    fn test_bytes_to_base64_with_symbols() {
        // "!\"#$%^&*()" -> "ISIjJCVeJiooKQ=="
        let input = b"!\"#$%^&*()";
        let expected = "ISIjJCVeJiooKQ==";
        assert_eq!(bytes_to_base64(input), expected);
    }

    #[test]
    fn test_bytes_to_base64_empty() {
        let input: &[u8] = b"";
        let expected = "";
        assert_eq!(bytes_to_base64(input), expected);
    }
}
