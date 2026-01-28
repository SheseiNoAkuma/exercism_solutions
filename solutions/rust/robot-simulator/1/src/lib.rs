// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
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
        match self.d {
            Direction::North => Robot::new(self.x, self.y, Direction::East),
            Direction::East => Robot::new(self.x, self.y, Direction::South),
            Direction::South => Robot::new(self.x, self.y, Direction::West),
            Direction::West => Robot::new(self.x, self.y, Direction::North),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        match self.d {
            Direction::North => Robot::new(self.x, self.y, Direction::West),
            Direction::East => Robot::new(self.x, self.y, Direction::North),
            Direction::South => Robot::new(self.x, self.y, Direction::East),
            Direction::West => Robot::new(self.x, self.y, Direction::South),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Robot::new(self.x, self.y + 1, self.d),
            Direction::East => Robot::new(self.x + 1, self.y, self.d),
            Direction::South => Robot::new(self.x, self.y - 1, self.d),
            Direction::West => Robot::new(self.x - 1, self.y, self.d),
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .into_iter()
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
