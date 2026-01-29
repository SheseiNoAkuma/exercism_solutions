pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut protein: Vec<&str> = Vec::new();

    for codon_byte in rna.as_bytes().chunks(3) {
        let codon = std::str::from_utf8(codon_byte).ok()?;

        if STOP.contains(&codon) {
            break;
        }
        to_amino_acid(codon).map(|n| protein.push(n))?;
    }

    Some(protein)
}
const STOP: [&str; 3] = ["UAA", "UAG", "UGA"];

fn to_amino_acid(codon: &str) -> Option<&str> {
    match codon {
        "AUG" => Some("Methionine"),
        "UUU" | "UUC" => Some("Phenylalanine"),
        "UUA" | "UUG" => Some("Leucine"),
        "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
        "UAU" | "UAC" => Some("Tyrosine"),
        "UGU" | "UGC" => Some("Cysteine"),
        "UGG" => Some("Tryptophan"),
        _ => None,
    }
}
