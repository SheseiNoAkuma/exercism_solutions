use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let word_map = to_char_map(word.to_lowercase().as_str());

    possible_anagrams
        .iter()
        .filter(|w| w.to_lowercase() != word.to_lowercase())
        .filter(|w| to_char_map(w.to_lowercase().as_str()) == word_map)
        .copied()
        .collect()
}

fn to_char_map(w: &str) -> HashMap<char, usize> {
    w.chars()
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}
