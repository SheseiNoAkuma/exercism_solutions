#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

struct BucketObj {
    capacity: u8,
    contain: u8,
    name: Bucket,
}

impl BucketObj {
    fn new(capacity: u8, name: Bucket) -> Self {
        BucketObj {
            capacity,
            contain: 0,
            name,
        }
    }

    fn is_empty(&self) -> bool {
        self.contain == 0
    }

    fn is_full(&self) -> bool {
        self.contain == self.capacity
    }

    fn left_capacity(&self) -> u8 {
        self.capacity - self.contain
    }

    fn has(&self) -> u8 {
        self.contain
    }

    fn full(&mut self) {
        self.contain = self.capacity
    }

    fn fill(&mut self, fill: u8, flag: bool) {
        if flag {
            self.contain += fill;
        } else {
            self.contain -= fill;
        }
    }

    fn empty(&mut self) {
        self.contain = 0
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut bucket_start: BucketObj;
    let mut bucket_other: BucketObj;
    if let Bucket::One = start_bucket {
        bucket_start = BucketObj::new(capacity_1, Bucket::One);
        bucket_other = BucketObj::new(capacity_2, Bucket::Two);
    } else {
        bucket_other = BucketObj::new(capacity_1, Bucket::One);
        bucket_start = BucketObj::new(capacity_2, Bucket::Two);
    }

    let mut times: u8 = 0;
    loop {
        times += 1;
        if bucket_start.is_full() && bucket_other.capacity == goal {
            bucket_other.full();
        } else if bucket_start.is_empty() {
            bucket_start.full();
        } else if bucket_other.is_full() {
            bucket_other.empty();
        } else if bucket_start.has() <= bucket_other.left_capacity() {
            bucket_other.fill(bucket_start.has(), true);
            bucket_start.empty();
        } else {
            bucket_start.fill(bucket_other.left_capacity(), false);
            bucket_other.full();
        }

        if bucket_start.has() == goal {
            return Some(BucketStats {
                moves: times,
                goal_bucket: bucket_start.name,
                other_bucket: bucket_other.has(),
            });
        }
        if bucket_other.has() == goal {
            return Some(BucketStats {
                moves: times,
                goal_bucket: bucket_other.name,
                other_bucket: bucket_start.has(),
            });
        }
        if bucket_start.is_full() && bucket_other.is_full() {
            return None;
        }
    }
}
