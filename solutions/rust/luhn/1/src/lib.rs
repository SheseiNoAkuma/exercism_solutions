/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    syntactic_validation(code) && semantic_validation(code)
}

fn syntactic_validation(code: &str) -> bool {
    code.trim().len() > 1
        && code
            .chars()
            .filter(is_not_space_char)
            .all(|c| c.is_digit(10))
}

fn semantic_validation(code: &str) -> bool {
    let sum: u32 = code
        .chars()
        .filter(is_not_space_char)
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(position, n)| {
            if position % 2 == 0 {
                n
            } else {
                double_and_check(n)
            }
        })
        .sum();

    sum % 10 == 0
}

fn double_and_check(number: u32) -> u32 {
    let doubled = number * 2;
    if doubled > 9 { doubled - 9 } else { doubled }
}

fn is_not_space_char(c: &char) -> bool {
    *c != ' '
}
