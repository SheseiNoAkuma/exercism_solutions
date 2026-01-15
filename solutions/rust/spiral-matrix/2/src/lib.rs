pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    let size = size as usize;

    let mut top = 0usize;
    let mut bottom = size.saturating_sub(1);
    let mut left = 0usize;
    let mut right = size.saturating_sub(1);

    let mut value = 1u32;

    while value <= (size * size) as u32 {
        (left..=right).for_each(|i| {
            matrix[top][i] = value;
            value += 1;
        });
        top += 1;
        (top..=bottom).for_each(|i| {
            matrix[i][right] = value;
            value += 1;
        });
        right = right.saturating_sub(1);
        if top <= bottom {
            (left..=right).rev().for_each(|i| {
                matrix[bottom][i] = value;
                value += 1;
            });
            bottom = bottom.saturating_sub(1);
        }
        if left <= right {
            (top..=bottom).rev().for_each(|i| {
                matrix[i][left] = value;
                value += 1;
            });
            left += 1;
        }
    }

    matrix
}
