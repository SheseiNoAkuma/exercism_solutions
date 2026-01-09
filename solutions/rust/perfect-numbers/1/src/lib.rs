use std::cmp::Ordering;
use std::iter::once;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else if num == 1 {
        return Some(Classification::Deficient);
    }

    let sum = (2..num)
        .filter(|&x| num % x == 0)
        .chain(once(1u64))
        .sum::<u64>();

    let classification = match num.cmp(&sum) {
        Ordering::Equal => Classification::Perfect,
        Ordering::Greater => Classification::Deficient,
        Ordering::Less => Classification::Abundant,
    };

    Some(classification)
}
