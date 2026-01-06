#[derive(Debug, PartialEq)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }

    fn attack_line<F>(&self, transform: F) -> Vec<ChessPosition>
    where
        F: Fn(i32) -> ChessPosition,
    {
        (0..8).map(transform).filter(|p| p != self).collect()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.attacked_positions().contains(&other.position)
    }

    fn attacked_positions(&self) -> Vec<ChessPosition> {
        let pos = &self.position;
        [
            pos.attack_line(|f| ChessPosition::new(pos.rank, f).unwrap()),
            pos.attack_line(|r| ChessPosition::new(r, pos.file).unwrap()),
            pos.attack_line(|i| {
                ChessPosition::new((pos.rank + i).rem_euclid(8), (pos.file + i).rem_euclid(8))
                    .unwrap()
            }),
            pos.attack_line(|i| {
                ChessPosition::new((pos.rank + i).rem_euclid(8), (pos.file - i).rem_euclid(8))
                    .unwrap()
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}
