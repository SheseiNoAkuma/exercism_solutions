pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let smallest_in_cols = smallest_in_column(input);
    largest_in_rows(input)
        .iter()
        .filter(|e| smallest_in_cols.contains(e))
        .cloned()
        .collect()
}
fn largest_in_rows(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row_values)| {
            row_values
                .iter()
                .max()
                .into_iter()
                .flat_map(move |&max_val| {
                    row_values
                        .iter()
                        .enumerate()
                        .filter(move |&(_, &val)| val == max_val)
                        .map(move |(col_idx, _)| (row_idx, col_idx))
                })
        })
        .collect()
}
fn smallest_in_column(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_count = input.len();
    if row_count == 0 {
        return Vec::new();
    }
    let col_count = input[0].len();

    (0..col_count)
        .flat_map(|col_idx| {
            let min_val = input.iter().map(|row| row[col_idx]).min();

            min_val.into_iter().flat_map(move |m| {
                input
                    .iter()
                    .enumerate()
                    .filter(move |(_, row)| row[col_idx] == m)
                    .map(move |(row_idx, _)| (row_idx, col_idx))
            })
        })
        .collect()
}
