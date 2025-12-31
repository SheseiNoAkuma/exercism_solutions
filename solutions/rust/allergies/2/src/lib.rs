use crate::Allergen::*;

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}
impl Allergen {
    const ALL: [Self; 8] = [
        Eggs,
        Peanuts,
        Shellfish,
        Strawberries,
        Tomatoes,
        Chocolate,
        Pollen,
        Cats,
    ];

    pub fn score(self) -> u32 {
        self as u32
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_score = Allergen::ALL
            .iter()
            .find(|a| *a == allergen)
            .map(|s| s.score())
            .unwrap_or(0);

        (self.score & allergen_score) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::ALL
            .iter()
            .filter_map(|a| Self::is_allergic_to(self, a).then_some(*a))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_allergic_to_cats_only() {
        let allergies = Allergies::new(128);
        assert!(!allergies.is_allergic_to(&Eggs));
        assert!(!allergies.is_allergic_to(&Peanuts));
        assert!(!allergies.is_allergic_to(&Shellfish));
        assert!(!allergies.is_allergic_to(&Strawberries));
        assert!(!allergies.is_allergic_to(&Tomatoes));
        assert!(!allergies.is_allergic_to(&Chocolate));
        assert!(!allergies.is_allergic_to(&Pollen));
        assert!(allergies.is_allergic_to(&Cats));
    }
    #[test]
    fn is_allergic_to_all() {
        let allergies = Allergies::new(255);
        assert!(allergies.is_allergic_to(&Eggs));
        assert!(allergies.is_allergic_to(&Peanuts));
        assert!(allergies.is_allergic_to(&Shellfish));
        assert!(allergies.is_allergic_to(&Strawberries));
        assert!(allergies.is_allergic_to(&Tomatoes));
        assert!(allergies.is_allergic_to(&Chocolate));
        assert!(allergies.is_allergic_to(&Pollen));
        assert!(allergies.is_allergic_to(&Cats));
    }
    #[test]
    fn is_allergic_to_none() {
        let allergies = Allergies::new(256);
        assert!(!allergies.is_allergic_to(&Eggs));
        assert!(!allergies.is_allergic_to(&Peanuts));
        assert!(!allergies.is_allergic_to(&Shellfish));
        assert!(!allergies.is_allergic_to(&Strawberries));
        assert!(!allergies.is_allergic_to(&Tomatoes));
        assert!(!allergies.is_allergic_to(&Chocolate));
        assert!(!allergies.is_allergic_to(&Pollen));
        assert!(!allergies.is_allergic_to(&Cats));
    }
}
