#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    dna: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    rna: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(invalid_index) = dna.chars().position(|c| !c.is_dna()) {
            return Err(invalid_index);
        }
        Ok(Dna {
            dna: dna.to_ascii_uppercase(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let rna: String = self.dna.chars().map(|c| c.to_dna()).collect();
        Rna::new(&rna).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(invalid_index) = rna.chars().position(|c| !c.is_rna()) {
            return Err(invalid_index);
        }
        Ok(Rna {
            rna: rna.to_ascii_uppercase(),
        })
    }
}

trait Nucleotide {
    fn is_dna(&self) -> bool;
    fn is_rna(&self) -> bool;
    fn to_dna(&self) -> Self;
}
impl Nucleotide for char {
    fn is_dna(&self) -> bool {
        match self.to_ascii_uppercase() {
            'A' | 'C' | 'G' | 'T' => true,
            _ => false,
        }
    }

    fn is_rna(&self) -> bool {
        match self.to_ascii_uppercase() {
            'A' | 'C' | 'G' | 'U' => true,
            _ => false,
        }
    }

    fn to_dna(&self) -> Self {
        match self.to_ascii_uppercase() {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => unreachable!("{} is not a valid DNA nucleotide", self),
        }
    }
}
