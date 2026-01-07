use std::collections::HashMap;

const DNA_CHARS: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    DNA_CHARS
        .iter()
        .find(|&&c| c == nucleotide)
        .ok_or(nucleotide)?;

    dna.chars()
        .find(|c| !DNA_CHARS.contains(c))
        .map_or(Ok(()), Err)?;

    Ok(dna.matches(nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    DNA_CHARS
        .iter()
        .map(|&nucleotide| match count(nucleotide, dna) {
            Ok(count) => Ok((nucleotide, count)),
            Err(n) => Err(n),
        })
        .collect::<Result<HashMap<_, _>, _>>()
}
