pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    digits
        .as_bytes()
        .windows(len)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect()
}
