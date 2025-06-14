use rand::Rng;

pub fn random_bytes(n: usize) -> Vec<u8> {
  let mut rng = rand::rng();
  (0..n).map(|_| rng.random_range(0..=255)).collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_random_bytes_length() {
    let bytes = random_bytes(16);
    assert_eq!(bytes.len(), 16);
  }

  #[test]
  fn test_random_bytes_randomness() {
    let a = random_bytes(8);
    let b = random_bytes(8);
    // 极小概率相等，这里只做基本不等性测试
    assert_ne!(a, b);
  }

  #[test]
  fn test_random_bytes_zero() {
    let bytes = random_bytes(0);
    assert_eq!(bytes.len(), 0);
  }
}
