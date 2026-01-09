const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let sentence = sentence.to_lowercase();
    ALPHABET.iter().all(|&c| sentence.contains(c))
}
