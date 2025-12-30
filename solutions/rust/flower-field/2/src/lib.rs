pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row, row_squares)| {
            row_squares
                .chars()
                .enumerate()
                .map(|(col, square)| map_square(square, || count_neighbors(row, col, garden)))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

fn map_square<F>(square: char, neighbors_count: F) -> String
where
    F: FnOnce() -> usize,
{
    match square {
        ' ' => {
            let count = neighbors_count();
            (count > 0)
                .then(|| count.to_string())
                .unwrap_or(" ".to_string())
        }
        '*' => "*".to_string(),
        _ => unreachable!("unknown square value: '{square:?}'"),
    }
}

fn count_neighbors(row: usize, col: usize, garden: &[&str]) -> usize {
    NEIGHBORS
        .iter()
        .filter_map(|(deviance_row, deviance_col)| {
            let adjacent_row = row as i32 + deviance_row;
            let adjacent_column = col as i32 + deviance_col;

            garden
                .get(adjacent_row as usize)
                .and_then(|row| row.chars().nth(adjacent_column as usize))
                .filter(|&square| square == '*')
                .map(|_| 1)
        })
        .count()
}

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_neighbors_edges() {
        let input = vec!["·*\n··\n··"];
        assert_eq!(count_neighbors(0, 0, &input), 1);
        assert_eq!(count_neighbors(0, 1, &input), 0);
        assert_eq!(count_neighbors(1, 0, &input), 1);
        assert_eq!(count_neighbors(1, 1, &input), 1);
        assert_eq!(count_neighbors(2, 0, &input), 0);
        assert_eq!(count_neighbors(2, 1, &input), 0);
    }
}
