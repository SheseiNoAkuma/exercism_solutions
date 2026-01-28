use std::ops::Not;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let state_initial = State::new(capacity_1, capacity_2).fill_bucket(start_bucket)?;

    let mut current_states: Vec<State> = [state_initial].into_iter().collect();
    println!("Initial state-> {:?}", current_states);

    let mut matched: Option<BucketStats> = find_match(&current_states, goal);

    while matched.is_none() && current_states.is_empty().not() {
        current_states = current_states
            .iter()
            .flat_map(|s| s.explode_options())
            .collect();

        matched = find_match(&current_states, goal);
    }

    matched
}

fn find_match(states: &Vec<State>, goal: u8) -> Option<BucketStats> {
    let mut matches: Vec<BucketStats> = states
        .iter()
        .filter_map(|s| s.to_goal(goal))
        .collect();

    println!("Matches-> {:?}", matches);

    matches.sort_by(|a, b| b.other_bucket.cmp(&a.other_bucket));

    println!("Matches after sort-> {:?}", matches);

    matches.first().copied()
}


#[derive(Debug, Copy, Clone)]
struct BucketImpl {
    capacity: u8,
    liters: u8,
    bucket: Bucket,
}
impl BucketImpl {
    fn new(capacity: u8, bucket: Bucket) -> Self {
        Self {
            capacity,
            liters: 0,
            bucket,
        }
    }

    fn fill(&mut self) -> Option<()> {
        self.is_full().not().then_some(self.liters = self.capacity)
    }

    fn empty(&mut self) -> Option<()> {
        self.is_empty().not().then_some(self.liters = 0)
    }

    fn poor(&mut self, target: &mut BucketImpl) -> Option<()> {
        if target.is_full() || self.is_empty() {
            return None;
        }

        if target.actual_capacity() > self.liters {
            target.liters += self.liters;
            self.empty();
        } else {
            self.liters -= target.actual_capacity();
            target.fill();
        }

        Some(())
    }

    fn actual_capacity(&self) -> u8 {
        self.capacity - self.liters
    }

    fn is_full(&self) -> bool {
        self.liters == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.liters == 0
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    bucker1: BucketImpl,
    bucker2: BucketImpl,
    current_move: u8,
}
impl State {
    fn new(capacity_1: u8, capacity_2: u8) -> Self {
        Self {
            bucker1: BucketImpl::new(capacity_1, Bucket::One),
            bucker2: BucketImpl::new(capacity_2, Bucket::Two),
            current_move: 0,
        }
    }

    fn add_move(&self) -> Self {
        Self {
            current_move: self.current_move + 1,
            ..*self
        }
    }

    fn fill_bucket(self, bucket: &Bucket) -> Option<Self> {
        let mut clone = self.clone();
        match bucket {
            Bucket::One => clone.bucker1.fill(),
            Bucket::Two => clone.bucker2.fill(),
        }
        .map(|_| clone.add_move())
    }

    fn empty_bucket(self, bucket: &Bucket) -> Option<Self> {
        let mut clone = self.clone();
        match bucket {
            Bucket::One => clone.bucker1.empty(),
            Bucket::Two => clone.bucker2.empty(),
        }
        .map(|_| clone.add_move())
    }

    fn poor_bucket(self, from: &Bucket) -> Option<Self> {
        let mut clone = self.clone();
        match from {
            Bucket::One => clone.bucker1.poor(&mut clone.bucker2),
            Bucket::Two => clone.bucker2.poor(&mut clone.bucker1),
        }
        .map(|_| clone.add_move())
    }

    fn explode_options(self) -> Vec<State> {
        [
            self.fill_bucket(&Bucket::One),
            self.fill_bucket(&Bucket::Two),
            self.empty_bucket(&Bucket::One),
            self.empty_bucket(&Bucket::Two),
            self.poor_bucket(&Bucket::One),
            self.poor_bucket(&Bucket::Two),
        ]
        .into_iter()
        .flatten()
        .filter(|s| s.should_stop().not())
        .collect()
    }

    fn should_stop(&self) -> bool {
        self.current_move >= 20 || (self.bucker1.is_empty() && self.bucker2.is_empty())
    }

    fn to_goal(&self, goal: u8) -> Option<BucketStats> {
        let goal_bucket = if self.bucker1.liters == goal {
            Some((self.bucker1, self.bucker2))
        } else if self.bucker2.liters == goal {
            Some((self.bucker2, self.bucker1))
        } else {
            None
        };

        goal_bucket.map(|(goal_b, other)| BucketStats {
            moves: self.current_move,
            goal_bucket: goal_b.bucket,
            other_bucket: other.liters,
        })
    }
}
