pub fn egg_count(display_value: u32) -> usize {
    let binary: String = format!("{display_value:b}");
    binary.chars().filter(|&c| c == '1').count()
}
