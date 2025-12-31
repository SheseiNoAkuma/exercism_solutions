pub fn abbreviate(phrase: &str) -> String {
    let words = phrase.split_whitespace().collect::<Vec<&str>>();

    words
        .iter()
        .flat_map(|word| split_camel_case(word))
        .flat_map(|word| word.split('-'))
        .map(|word| word.chars().filter(|c| c.is_alphabetic()).next().unwrap_or(' '))
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>()
}

fn split_camel_case(s: &str) -> Vec<&str> {
    if s == s.to_uppercase() {
        return vec![s];
    }

    let mut result = Vec::new();
    let mut start = 0;

    for (i, c) in s.char_indices() {
        if i != 0 && c.is_uppercase() {
            result.push(&s[start..i]);
            start = i;
        }
    }

    result.push(&s[start..]);
    result
}