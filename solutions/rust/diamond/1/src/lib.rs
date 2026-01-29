pub fn get_diamond(c: char) -> Vec<String> {
    let n = distance('A', c); // 1..=N
    let rows = 2 * n - 1;
    let mid = n - 1; // 0-based center

    (0..rows)
        .map(|r| {
            let dv = mid.abs_diff(r);
            let level = mid - dv; // 0..=mid
            let ch = (b'A' + level as u8) as char;

            let dx = level;
            let left = mid - dx;
            let right = mid + dx;

            (0..rows)
                .map(|col| if col == left || col == right { ch } else { ' ' })
                .collect::<String>()
        })
        .collect()
}

fn distance(a: char, b: char) -> usize {
    (b as u8 - a as u8) as usize + 1
}
