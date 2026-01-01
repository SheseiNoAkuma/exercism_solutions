pub fn collatz(n: u64) -> Option<u64> {
    fn collatz_recursive(n: u64, steps: u64) -> Option<u64> {
        match n {
            0 => None,
            1 => Some(steps),
            n if n % 2 == 0 => collatz_recursive(n / 2, steps + 1),
            _ => collatz_recursive(3 * n + 1, steps + 1),
        }
    }
    collatz_recursive(n, 0)
}
