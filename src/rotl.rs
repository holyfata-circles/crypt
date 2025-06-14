pub fn rotl(n: u32, b: u32) -> u32 {
    n.rotate_left(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotl_basic() {
        assert_eq!(rotl(0x12345678, 8), 0x34567812);
        assert_eq!(rotl(0x12345678, 0), 0x12345678);
        assert_eq!(rotl(0x12345678, 32), 0x12345678);
    }
}