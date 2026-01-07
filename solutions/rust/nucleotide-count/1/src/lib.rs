use std::collections::HashMap;

const DNA_CHARS: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !DNA_CHARS.contains(&nucleotide) {
        return Err(nucleotide);
    }
    let invalid_char = dna.chars().find(|c| !DNA_CHARS.contains(c));
    if invalid_char.is_some() {
        return Err(invalid_char.unwrap());
    }

    let count = dna.chars().filter(|c| *c == nucleotide).count();
    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    DNA_CHARS
        .iter()
        .map(|&nucleotide| count(nucleotide, dna).map(|count| (nucleotide, count)))
        .collect::<Result<HashMap<_, _>, _>>()
}
