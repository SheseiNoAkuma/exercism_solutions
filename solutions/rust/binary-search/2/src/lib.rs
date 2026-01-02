pub fn find<C, K>(array: C, key: K) -> Option<usize>
where
    C: AsRef<[K]>,
    K: Ord,
{
    let array = array.as_ref();

    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let middle = (left + right) / 2;
        match array[middle].cmp(&key) {
            std::cmp::Ordering::Less => left = middle + 1,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Greater => right = middle,
        }
    }
    None
}
