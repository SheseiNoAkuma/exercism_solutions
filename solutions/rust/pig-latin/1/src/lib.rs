pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(translate_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn translate_word(word: &str) -> String {
    match word {
        _ if matches_rule_1(word) => word.to_string() + "ay",
        _ if matches_rule_3(word) => rule_3_reorder(word).unwrap_or(word.to_string()) + "ay",
        _ if matches_rule_4(word) => word
            .find("y")
            .map(|i| word[i..].to_owned() + &word[..i] + "ay")
            .expect("match rule 4 should always return Some"),

        _ if matches_rule_2(word) => {
            let x = split_once_keep(word, |c: char| c.is_vowel());
            x.map(|(p1, p2)| p2.to_owned() + p1 + "ay")
                .unwrap_or(word.to_string())
        }

        _ => word.to_string(),
    }
}

fn matches_rule_1(input: &str) -> bool {
    match input.to_ascii_lowercase().chars().next() {
        Some(c) if c.is_vowel() => true,
        _ if input.starts_with("xr") || input.starts_with("yt") => true,
        _ => false,
    }
}

fn matches_rule_2(input: &str) -> bool {
    let fist_char = input.to_ascii_lowercase().chars().next();

    match fist_char {
        Some(c) => c.is_consonant(),
        _ => false,
    }
}

fn matches_rule_3(input: &str) -> bool {
    let contains_qu = input.to_ascii_lowercase().find("qu");
    match contains_qu {
        Some(0) => true,
        Some(n) => input[..n].chars().all(|c| c.is_consonant()),
        _ => false,
    }
}

fn matches_rule_4(input: &str) -> bool {
    match input.find("y") {
        Some(0) => false,
        Some(n) => input[..n].chars().all(|c| c.is_consonant()),
        _ => false,
    }
}

fn split_once_keep<F>(s: &str, pred: F) -> Option<(&str, &str)>
where
    F: Fn(char) -> bool,
{
    let (idx, _) = s.char_indices().find(|&(_, c)| pred(c))?;
    Some((&s[..idx], &s[idx..]))
}

fn rule_3_reorder(s: &str) -> Option<String> {
    match s.find("qu") {
        Some(0) => Some(s[2..].to_owned() + "qu"),
        Some(n) => Some(s[n + 2..].to_owned() + &s[0..n] + "qu"),
        _ => None,
    }
}

trait Vowel {
    fn is_vowel(&self) -> bool;
    fn is_consonant(&self) -> bool;
}
impl Vowel for char {
    fn is_vowel(&self) -> bool {
        matches!(self, 'a' | 'e' | 'i' | 'o' | 'u')
    }

    fn is_consonant(&self) -> bool {
        self.is_alphabetic() && !self.is_vowel()
    }
}
