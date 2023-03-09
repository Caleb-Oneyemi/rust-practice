fn greatest_common_divisor(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 && b != 0);

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

#[test]
fn test_gcd() {
    assert_eq!(greatest_common_divisor(14, 15), 1);

    assert_eq!(greatest_common_divisor(54, 24), 6);

    assert_eq!(greatest_common_divisor(5610, 57057), 3 * 11);
}

use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("input must be a number"))
    }

    if numbers.len() == 0 {
        eprintln!("Usage: cargo run n1 n2 n3 ...");
        std::process::exit(1)
    }

    let mut d = numbers[0];
    for num in &numbers[1..] {
        d = greatest_common_divisor(d, *num)
    }

    println!("gcd of {:?} is {}", numbers, d)
}
