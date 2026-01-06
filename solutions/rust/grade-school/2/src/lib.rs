use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let already_exists = self.grades
            .iter()
            .any(|(_, v)| v.contains(&student.to_string()));

        if already_exists {
            println!("Student already exists");
            return
        }

        let current_grade = self.grades.entry(grade).or_default();
        current_grade.insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().map(|k| *k).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .iter()
            .find(|(k, _)| **k == grade)
            .map(|(_, v)| v)
            .unwrap_or(&BTreeSet::new())
            .iter()
            .cloned()
            .collect()
    }
}
