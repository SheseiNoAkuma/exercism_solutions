pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut protein: Vec<&str> = Vec::new();
    let bytes = rna.as_bytes();

    for i in (0..bytes.len()).step_by(3) {
        let upper_limit = bytes.len().min(i + 3);
        let codon = std::str::from_utf8(&bytes[i..upper_limit]).ok()?;

        if is_stop_codon(codon) {
            break;
        }
        is_amniotic_dna(codon).then(|| protein.push(to_extended_name(codon)))?;
    }

    Some(protein)
}

fn to_extended_name(amnions: &str) -> &str {
    match amnions {
        "AUG" => "Methionine",
        "UUU" | "UUC" => "Phenylalanine",
        "UUA" | "UUG" => "Leucine",
        "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
        "UAU" | "UAC" => "Tyrosine",
        "UGU" | "UGC" => "Cysteine",
        "UGG" => "Tryptophan",
        _ => unreachable!("Invalid codon"),
    }
}

const AMNIONS: [&str; 14] = [
    "AUG", "UUU", "UUC", "UUA", "UUG", "UCU", "UCC", "UCA", "UCG", "UAU", "UAC", "UGU", "UGC",
    "UGG",
];

const STOP: [&str; 3] = ["UAA", "UAG", "UGA"];

fn is_amniotic_dna(dna: &str) -> bool {
    AMNIONS.contains(&dna)
}
fn is_stop_codon(dna: &str) -> bool {
    STOP.contains(&dna)
}