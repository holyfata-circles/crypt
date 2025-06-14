pub fn rotr(n: u32, b: u32) -> u32 {
    n.rotate_right(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotr_basic() {
        assert_eq!(rotr(0x12345678, 8), 0x78123456);
        assert_eq!(rotr(0x12345678, 0), 0x12345678);
        assert_eq!(rotr(0x12345678, 32), 0x12345678);
    }
}