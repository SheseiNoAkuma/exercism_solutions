pub fn square(s: u32) -> u64 {
    assert!(
        s > 0 && s <= 64,
        "only positive numbers up to 64 are valid squares"
    );
    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
