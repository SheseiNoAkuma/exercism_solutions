/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn_numbers: Vec<u32> = isbn
        .chars()
        .filter(valid_isbn_chars)
        .enumerate()
        .flat_map(|(position, char)| convert_char_to_digit(char, position))
        .collect();

    if isbn_numbers.len() != 10 || isbn.chars().filter(|c| *c != '-').count() != 10 {
        return false;
    }

    let sum = (1..=10)
        .rev()
        .enumerate()
        .map(|(index, number)| isbn_numbers.get(index).unwrap_or(&10) * number)
        .sum::<u32>();

    sum % 11 == 0
}

fn convert_char_to_digit(c: char, position: usize) -> Option<u32> {
    if c == 'X' && position == 9 {
        Some(10)
    } else if c.is_numeric() {
        c.to_digit(10)
    } else {
        None
    }
}

fn valid_isbn_chars(c: &char) -> bool {
    c.is_numeric() || *c == 'X'
}
