pub fn square(s: u32) -> u64 {
    assert!(s > 0, "square must be positive");
    assert!(s <= 64, "square does not feet on chessboard");
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
