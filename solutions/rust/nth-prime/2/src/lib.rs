pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}

fn is_prime(n: u32) -> bool {
    (2..n).all(|i| n % i != 0)
}
