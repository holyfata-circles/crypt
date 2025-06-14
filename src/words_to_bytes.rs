pub fn words_to_bytes(words: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for b in (0..words.len() * 32).step_by(8) {
        let word = words[b >> 5];
        let byte = ((word >> (24 - (b % 32))) & 0xFF) as u8;
        bytes.push(byte);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_to_bytes_basic() {
        // 0x4d616e48 = [0x4d, 0x61, 0x6e, 0x48]
        let words = [0x4d616e48];
        assert_eq!(words_to_bytes(&words), vec![0x4d, 0x61, 0x6e, 0x48]);
    }

    #[test]
    fn test_words_to_bytes_multiple_words() {
        let words = [0x48656c6c, 0x6f212223];
        // "Hello!\"#"
        assert_eq!(
            words_to_bytes(&words),
            vec![0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x22, 0x23]
        );
    }

    #[test]
    fn test_words_to_bytes_empty() {
        let words: [u32; 0] = [];
        assert_eq!(words_to_bytes(&words), vec![]);
    }
}