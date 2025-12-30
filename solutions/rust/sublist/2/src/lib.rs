use crate::Comparison::{Equal, Sublist, Superlist, Unequal};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Equal
    } else if first_list.is_superlist_of(second_list) {
        Superlist
    } else if second_list.is_superlist_of(first_list) {
        Sublist
    } else {
        Unequal
    }
}

trait SliceSublistExt<T> {
    fn is_superlist_of(&self, other: &[T]) -> bool;
}
impl<T: PartialEq> SliceSublistExt<T> for [T] {
    fn is_superlist_of(&self, other: &[T]) -> bool {
        match other.len() {
            _ if self.len() <= other.len() => false,
            0 => true,
            _ => self.windows(other.len()).any(|w| w == other),
        }
    }
}
