use std::collections::{HashMap, HashSet};

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
        product(*self.pairs.first().unwrap())
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.pairs.into_iter().collect()
    }
}

fn is_palindrome(tuple: &(u64, u64)) -> bool {
    let digits = (tuple.0 * tuple.1).to_string();
    digits == digits.chars().rev().collect::<String>()
}

fn product(tuple: (u64, u64)) -> u64 {
    tuple.0 * tuple.1
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let range = min..=max;
    let cartesian: Vec<_> = range
        .clone()
        .flat_map(|x| range.clone().filter(move |&y| y >= x).map(move |y| (x, y)))
        .filter(is_palindrome)
        .fold(HashMap::<u64, Vec<(u64, u64)>>::new(), |mut acc, pair| {
            let product = product(pair);
            let mut pairs_with_same_product = acc.get(&product).cloned().unwrap_or_else(Vec::new);

            pairs_with_same_product.push(pair);
            acc.insert(product, pairs_with_same_product);
            acc
        })
        .values()
        .map(|x| Palindrome::new(x))
        .collect();

    cartesian
        .iter()
        .min()
        .zip(cartesian.iter().max())
        .map(|(a, b)| (a.clone(), b.clone()))
}
