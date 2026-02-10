use rand::distr::{Distribution, Uniform};
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    used_names: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot {
    name: String,
    used_names: Rc<RefCell<HashSet<String>>>,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            used_names: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Robot {
        let name = Self::generate_unique_name(&self.used_names, rng);
        Robot {
            name,
            used_names: Rc::clone(&self.used_names),
        }
    }

    fn generate_unique_name<R: Rng + ?Sized>(
        used_names: &Rc<RefCell<HashSet<String>>>,
        rng: &mut R,
    ) -> String {
        loop {
            let candidate = Self::generate_random_name(rng);
            // Insert returns true if the value was not present.
            if used_names.borrow_mut().insert(candidate.clone()) {
                return candidate;
            }
            // Collision: try again (consumes more RNG, but guarantees uniqueness).
        }
    }

    fn generate_random_name<R: Rng + ?Sized>(rng: &mut R) -> String {
        let letters = Uniform::new_inclusive(b'A', b'Z').unwrap();
        let digits = Uniform::new_inclusive(0u8, 9u8).unwrap();

        let mut s = String::with_capacity(5);

        for _ in 0..2 {
            s.push(letters.sample(rng) as char);
        }
        for _ in 0..3 {
            s.push((b'0' + digits.sample(rng)) as char);
        }

        s
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        // Release the current name back to the factory’s pool.
        self.used_names.borrow_mut().remove(&self.name);

        // Acquire a new unique name.
        self.name = RobotFactory::generate_unique_name(&self.used_names, rng);
    }
}

impl Drop for Robot {
    fn drop(&mut self) {
        // When a robot goes away, its name becomes available again.
        self.used_names.borrow_mut().remove(&self.name);
    }
}
