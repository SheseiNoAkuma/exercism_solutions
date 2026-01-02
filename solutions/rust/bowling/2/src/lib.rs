#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
struct Frame {
    shoot1: u16,
    shoot2: Option<u16>,
    is_last_frame: bool,
    bonus: ExtraRoll,
}
impl Frame {
    fn new(shoot1: u16, is_last_frame: bool) -> Self {
        Frame {
            shoot1,
            shoot2: None,
            is_last_frame,
            bonus: ExtraRoll::new(),
        }
    }

    fn is_strike(&self) -> bool {
        self.shoot2.is_none() && self.shoot1 == 10
    }

    fn is_spare(&self) -> bool {
        self.shoot2.is_some() && self.total_pins() == 10
    }

    fn is_open(&self) -> bool {
        self.total_pins() < 10
    }

    fn is_completed(&self) -> bool {
        match (self.is_open(), self.is_last_frame) {
            (true, _) => self.shoot2.is_some(),
            (_, true) if self.is_strike() => self.bonus.is_full(),
            (_, true) if self.is_spare() => self.bonus.is_partial(),
            _ => true,
        }
    }

    fn total_pins(&self) -> u16 {
        self.shoot1 + self.shoot2.unwrap_or(0)
    }

    fn add_shoot(&mut self, pins: u16) -> Result<(), Error> {
        if !self.is_strike() && self.shoot2.is_none() {
            if self.shoot1 + pins > 10 {
                return Err(Error::NotEnoughPinsLeft);
            }
            self.shoot2 = Some(pins);
            Ok(())
        } else {
            self.bonus.push(pins)
        }
    }

    fn maybe_shoot_two(&self) -> Option<u16> {
        self.shoot2.or(self.bonus.one)
    }

    fn score(&self, following: &[Frame]) -> u16 {
        if following.is_empty() {
            self.total_pins() + self.bonus.score()
        } else if self.is_open() {
            self.total_pins()
        } else if self.is_spare() {
            self.total_pins() + following[0].shoot1
        } else if self.is_strike() {
            let shoot2 = following[0]
                .maybe_shoot_two()
                .or_else(|| following.get(1).map(|f| f.shoot1))
                .unwrap_or(0);
            self.total_pins() + following[0].shoot1 + shoot2
        } else {
            unreachable!("Invalid frame state:")
        }
    }
}
#[derive(Debug)]
struct ExtraRoll {
    one: Option<u16>,
    two: Option<u16>,
}
impl ExtraRoll {
    fn new() -> Self {
        ExtraRoll {
            one: None,
            two: None,
        }
    }

    fn push(&mut self, pins: u16) -> Result<(), Error> {
        match self.one {
            Some(_) => self.two = Some(pins),
            None => self.one = Some(pins),
        }
        if !self.is_valid_score() {
            return Err(Error::NotEnoughPinsLeft);
        }
        Ok(())
    }

    fn is_valid_score(&self) -> bool {
        match (self.one, self.two) {
            (Some(x), Some(10)) if x < 10 => false,
            (Some(x), Some(y)) if x + y > 10 && (x != 10 && y != 10) => false,
            _ => true,
        }
    }

    fn is_full(&self) -> bool {
        self.one.is_some() && self.two.is_some()
    }

    fn is_partial(&self) -> bool {
        self.one.is_some()
    }

    fn score(&self) -> u16 {
        self.one.unwrap_or(0) + self.two.unwrap_or(0)
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
}
impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::with_capacity(10),
        }
    }

    fn is_game_ended(&self) -> bool {
        self.frames.len() == 10 && self.frames.last().unwrap().is_completed()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_ended() {
            return Err(Error::GameComplete);
        } else if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let maybe_in_progress_frame = self.frames.iter_mut().find(|f| !f.is_completed());

        if maybe_in_progress_frame.is_some() {
            maybe_in_progress_frame.unwrap().add_shoot(pins)?;
        } else {
            let f = Frame::new(pins, self.frames.len() == 9);
            self.frames.push(f);
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_game_ended() {
            return None;
        }
        let score: u16 = (0..10)
            .rev()
            .map(|i| {
                let frame = &self.frames[i];
                frame.score(&self.frames[(i + 1)..])
            })
            .sum();

        Some(score)
    }
}
