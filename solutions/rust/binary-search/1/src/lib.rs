pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut array: Vec<(usize, i32)> = array.iter().enumerate().map(|(i, &v)| (i, v)).collect();

    while !array.is_empty() {
        let middle = array.len() / 2;
        if array[middle].1 == key {
            return Some(array[middle].0);
        } else if middle == 0 {
            return None;
        }

        if array[middle].1 > key {
            array.drain(middle..);
        } else {
            array.drain(..middle);
        }
    }
    None
}
