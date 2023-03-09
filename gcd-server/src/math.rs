pub fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
  while b != 0 {
      if b < a {
          let temp= b;
          b = a;
          a = temp;
      }
      b = b % a;
  }
  a
}
