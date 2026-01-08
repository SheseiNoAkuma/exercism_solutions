/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let sentence = sentence.to_lowercase();
    (b'a'..=b'z').all(|c| sentence.contains(c as char))
}
