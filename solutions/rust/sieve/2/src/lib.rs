pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut all_numbers = (2..=upper_bound).collect::<Vec<u64>>();
    let mut primes = vec![];

    while let Some(&next_prime) = all_numbers.first() {
        primes.push(next_prime);
        all_numbers.retain(|&x| x % next_prime != 0);
    }

    primes
}
