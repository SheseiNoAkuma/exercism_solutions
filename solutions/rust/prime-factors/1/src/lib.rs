pub fn factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }

    let mut factors: Vec<u64> = vec![];
    let mut current_factor: u64 = 2;
    let mut current_number: u64 = n;

    loop {
        if is_prime(&current_number) {
            factors.push(current_number);
            return factors;
        }
        if (current_number % current_factor) == 0 {
            factors.push(current_factor);
            current_number = current_number / current_factor;
        } else {
            current_factor += 1;
        }
    }
}

fn is_prime(n: &u64) -> bool {
    if n < &2 {
        return false;
    }
    if n == &2 || n == &3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let limit = (*n as f64).sqrt() as u64;
    (3..=limit).step_by(2).all(|i| n % i != 0)
}
