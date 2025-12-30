use std::cmp::PartialEq;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let garden_as_model: Vec<Vec<Square>> = garden
        .iter()
        .map(|row| row.chars().map(Square::from).collect::<Vec<_>>())
        .collect();

    let garden_transformed: Vec<Vec<Square>> = garden_as_model
        .iter()
        .enumerate()
        .map(|(row, row_squares)| {
            row_squares
                .iter()
                .enumerate()
                .map(|(col, square)| match square {
                    Square::Empty => Square::Neighbors {
                        number_of_flowers: count_neighbors(row, col, &garden_as_model) as u8,
                    },
                    Square::Flower => Square::Flower,
                    _ => unreachable!("unknown square value: '{square:?}'"),
                })
                .collect()
        })
        .collect();

    pretty_print(&garden_transformed)
}

fn pretty_print(garden: &[Vec<Square>]) -> Vec<String> {
    garden
        .iter()
        .map(|row| {
            row.iter()
                .map(|square| match square {
                    Square::Flower => '*',
                    Square::Neighbors { number_of_flowers } if number_of_flowers > &0u8 => {
                        number_of_flowers.to_string().chars().next().unwrap()
                    }
                    _ => ' ',
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

fn count_neighbors(row: usize, col: usize, garden: &[Vec<Square>]) -> usize {
    NEIGHBORS
        .iter()
        .filter_map(|(deviance_row, deviance_col)| {
            let adjacent_row = row as i32 + deviance_row;
            let adjacent_column = col as i32 + deviance_col;

            if adjacent_row < 0 || adjacent_column < 0 {
                return None;
            }

            garden
                .get(adjacent_row as usize)
                .and_then(|row| row.get(adjacent_column as usize))
                .filter(|c| **c == Square::Flower)
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

#[derive(Debug, PartialEq)]
enum Square {
    Empty,
    Flower,
    Neighbors { number_of_flowers: u8 },
}
impl From<char> for Square {
    fn from(value: char) -> Self {
        match value {
            ' ' => Square::Empty,
            '*' => Square::Flower,
            _ => unreachable!("unknown square value: '{value:?}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_neighbors_edges() {
        let input = vec![
            vec![Square::Empty, Square::Flower],
            vec![Square::Empty, Square::Empty],
            vec![Square::Empty, Square::Empty],
        ];
        assert_eq!(count_neighbors(0, 0, &input), 1);
        assert_eq!(count_neighbors(0, 1, &input), 0);
        assert_eq!(count_neighbors(1, 0, &input), 1);
        assert_eq!(count_neighbors(1, 1, &input), 1);
        assert_eq!(count_neighbors(2, 0, &input), 0);
        assert_eq!(count_neighbors(2, 1, &input), 0);
    }
}
