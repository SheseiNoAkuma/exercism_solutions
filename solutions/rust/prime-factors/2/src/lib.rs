pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut current_factor: u64 = 2;
    let mut remaining: u64 = n;

    while remaining > 1 {
        if is_prime(remaining) {
            factors.push(remaining);
            break;
        }
        if (remaining % current_factor) == 0 {
            factors.push(current_factor);
            remaining /= current_factor;
        } else if current_factor == 2 {
            current_factor = 3;
        } else {
            current_factor += 2;
        }
    }

    factors
}

fn is_prime(number: u64) -> bool {
    if number % 2 == 0 && number != 2 || number == 1 {
        return false;
    }
    let limit = (number as f64).sqrt() as u64 + 1;
    // We test if the number is divisible by any odd number up to the limit
    (3..limit).step_by(2).all(|x| number % x != 0)
}
