pub fn is_armstrong_number(num: u32) -> bool {
    let digits = digits(num);
    let len = digits.len();

    digits.iter().map(|d| d.pow(len as u32)).sum::<u32>() == num
}

fn digits(num: u32) -> Vec<u32> {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
