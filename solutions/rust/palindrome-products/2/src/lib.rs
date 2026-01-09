use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    pairs: Vec<(u64, u64)>,
}
impl Ord for Palindrome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}
impl PartialOrd for Palindrome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Palindrome {
    pub fn new(pairs: &[(u64, u64)]) -> Self {
        Palindrome {
            pairs: pairs.to_vec(),
        }
    }

    pub fn value(&self) -> u64 {
        product(self.pairs.first().unwrap())
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.pairs.into_iter().collect()
    }
}

fn is_palindrome(tuple: &(u64, u64)) -> bool {
    let digits = (tuple.0 * tuple.1).to_string();
    digits == digits.chars().rev().collect::<String>()
}

fn product(tuple: &(u64, u64)) -> u64 {
    tuple.0 * tuple.1
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_factors: Vec<(u64, u64)> = vec![];
    let mut max_factors: Vec<(u64, u64)> = vec![];

    for i in min..=max {
        for j in min..=max {
            let current_pair = (i, j);
            let current_product = product(&current_pair);
            if j < i || !is_palindrome(&current_pair) {
                continue;
            }
            match min_factors.first().map(product) {
                Some(current_min) if current_min < current_product => (),
                Some(current_min) if current_min > current_product => {
                    min_factors.clear();
                    min_factors.push(current_pair)
                }
                _ => min_factors.push(current_pair),
            }
            match max_factors.first().map(product) {
                Some(current_max) if current_max > current_product => (),
                Some(current_max) if current_max < current_product => {
                    max_factors.clear();
                    max_factors.push(current_pair)
                }
                _ => max_factors.push(current_pair),
            }
        }
    }
    println!("{:?}", min_factors);
    println!("{:?}", max_factors);

    match min_factors.first().zip(max_factors.first()) {
        Some(_) => Some((Palindrome::new(&min_factors), Palindrome::new(&max_factors))),
        _ => None,
    }
}
