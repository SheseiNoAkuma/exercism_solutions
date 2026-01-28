// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests, you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn to_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn to_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Robot::new(self.x, self.y, self.d.to_right())
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot::new(self.x, self.y, self.d.to_left())
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = match self.d {
            Direction::North => (self.x, self.y + 1),
            Direction::East => (self.x + 1, self.y),
            Direction::South => (self.x, self.y - 1),
            Direction::West => (self.x - 1, self.y),
        };

        Robot::new(x, y, self.d)
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .map(Instruction::from)
            .fold(self, Self::execute_instruction)
    }

    fn execute_instruction(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::TurnRight => self.turn_right(),
            Instruction::TurnLeft => self.turn_left(),
            Instruction::Advance => self.advance(),
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

enum Instruction {
    TurnRight,
    TurnLeft,
    Advance,
}
impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'R' => Instruction::TurnRight,
            'L' => Instruction::TurnLeft,
            'A' => Instruction::Advance,
            _ => panic!("Invalid instruction: {}", value),
        }
    }
}
