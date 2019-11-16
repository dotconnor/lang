pub fn create_memory(size_in_bytes: u16) -> Vec<u8> {
  return vec![0; size_in_bytes as usize];
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_memory() {
    let m = create_memory(2);
    assert_eq!(m.len(), 2);
  }
}
