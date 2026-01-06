pub fn egg_count(display_value: u32) -> usize {
    let binary: String = format!("{:b}", display_value);
    binary.chars().filter(|&c| c == '1').count()
}
