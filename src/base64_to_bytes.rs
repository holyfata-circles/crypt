use crate::BASE64_MAP;
use crate::clean_base64;

pub fn base64_to_bytes(base64_str: String) -> Vec<u8> {
    let base64_str = clean_base64(&base64_str);
    let base64map = BASE64_MAP;
    let mut bytes = Vec::new();
    let chars: Vec<char> = base64_str.chars().collect();
    let len = chars.len();

    // 修复：避免负移位和溢出，且逻辑更清晰
    for i in 1..len {
        let imod4 = i % 4;
        if imod4 == 0 {
            continue;
        }
        let prev = base64map.find(chars[i - 1]).unwrap_or(0) as u8;
        let curr = base64map.find(chars[i]).unwrap_or(0) as u8;

        let left_mask = (1u8 << (8 - 2 * imod4)) - 1;
        let left = (prev & left_mask) << (imod4 * 2);
        let right = curr >> (6 - imod4 * 2);
        bytes.push(left | right);
    }

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_to_bytes_simple() {
        // "TWFu" -> "Man"
        let input = "TWFu";
        let expected = b"Man".to_vec();
        assert_eq!(base64_to_bytes(input.to_string()), expected);
    }

    #[test]
    fn test_base64_to_bytes_with_padding() {
        // "TWE=" -> "Ma"
        let input = "TWE=";
        let expected = b"Ma".to_vec();
        assert_eq!(base64_to_bytes(input.to_string()), expected);

        // "TQ==" -> "M"
        let input2 = "TQ==";
        let expected2 = b"M".to_vec();
        assert_eq!(base64_to_bytes(input2.to_string()), expected2);
    }

    #[test]
    fn test_base64_to_bytes_with_invalid_chars() {
        // "T W F u" -> "Man"
        let input = "T W F u";
        let expected = b"Man".to_vec();
        assert_eq!(base64_to_bytes(input.to_string()), expected);
    }

    #[test]
    fn test_base64_to_bytes_empty() {
        let input = "";
        let expected: Vec<u8> = vec![];
        assert_eq!(base64_to_bytes(input.to_string()), expected);
    }
}
