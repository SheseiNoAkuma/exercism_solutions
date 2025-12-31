/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect::<Vec<char>>();

    if code.len() < 2 || code.iter().any(|c| !c.is_numeric()) {
        return false;
    }

    let sum = code
        .iter()
        .map(|c| c.to_digit(10).unwrap())
        .rev()
        .enumerate()
        .fold(0, |sum, (position, c)| {
            if position % 2 == 0 {
                sum + c
            } else {
                sum + double_and_check(c)
            }
        });

    sum % 10 == 0
}

fn double_and_check(number: u32) -> u32 {
    let doubled = number * 2;
    if doubled > 9 { doubled - 9 } else { doubled }
}
