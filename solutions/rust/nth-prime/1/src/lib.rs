pub fn nth(n: u32) -> u32 {
    let mut prime_foud = 0;
    let mut current = 2;

    loop {
        let prime = is_prime(current);
        if prime && prime_foud == n {
            return current;
        }
        prime_foud += prime as u32;
        current += 1;
    }
}

fn is_prime(n: u32) -> bool {
    (2..n).all(|i| n % i != 0)
}
