pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.row_count as usize).fold(Vec::new(), |mut acc: Vec<Vec<u32>>, i| {
            acc.push(add_row(i, acc.last()));
            acc
        })
    }
}

fn add_row(index: usize, last_row: Option<&Vec<u32>>) -> Vec<u32> {
    (0..index)
        .map(|i| {
            let next: Option<&u32> = last_row.and_then(|row| row.get(i));
            let prev: Option<&u32> = last_row.filter(|_| i > 0).and_then(|row| row.get(i - 1));

            next.and_then(|next| prev.map(|prev| prev + next))
                .unwrap_or(1)
        })
        .collect()
}
