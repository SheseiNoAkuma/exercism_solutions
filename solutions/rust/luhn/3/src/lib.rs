pub fn is_valid(code: &str) -> bool {
    let result: Result<_, String> = code
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .enumerate()
        .try_fold((0u32, 0usize), |(acc, size), (position, number)| {
            let number = number.to_digit(10).ok_or(format!("{number} not a digit"))?;
            Ok((acc + luhn_double(number, position), size + 1))
        });

    match result {
        Ok((sum, size)) => sum % 10 == 0 && size >= 2,
        other => {
            println!("not valid cause -> {:?}", other);
            false
        }
    }
}

fn luhn_double(number: u32, position: usize) -> u32 {
    if position % 2 == 0 {
        number
    } else {
        let doubled = number * 2;
        if doubled > 9 { doubled - 9 } else { doubled }
    }
}
