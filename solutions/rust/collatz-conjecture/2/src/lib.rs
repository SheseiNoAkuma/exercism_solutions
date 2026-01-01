pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut n = n;

    loop {
        if n == 1 {
            return Some(steps);
        }
        steps += 1;
        if (n % 2) == 0 { n /= 2 } else { n = 3 * n + 1 }
    }
}
