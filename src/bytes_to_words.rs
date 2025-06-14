pub fn bytes_to_words(bytes: &[u8]) -> Vec<u32> {
    let mut words = vec![0u32; bytes.len().div_ceil(4)];
    let mut b = 0;
    for &byte in bytes.iter() {
        let word_index = b >> 5;
        let shift = 24 - (b % 32);
        words[word_index] |= (byte as u32) << shift;
        b += 8;
    }
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_words_basic() {
        let bytes = [0x4d, 0x61, 0x6e, 0x48];
        assert_eq!(bytes_to_words(&bytes), vec![0x4d616e48]);
    }

    #[test]
    fn test_bytes_to_words_multiple_words() {
        let bytes = [0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x21, 0x22, 0x23];
        assert_eq!(bytes_to_words(&bytes), vec![0x48656c6c, 0x6f212223]);
    }

    #[test]
    fn test_bytes_to_words_empty() {
        let bytes: [u8; 0] = [];
        assert_eq!(bytes_to_words(&bytes), vec![]);
    }

    #[test]
    fn test_bytes_to_words_not_multiple_of_4() {
        let bytes = [0x12, 0x34, 0x56];
        assert_eq!(bytes_to_words(&bytes), vec![0x12345600]);
    }
}
