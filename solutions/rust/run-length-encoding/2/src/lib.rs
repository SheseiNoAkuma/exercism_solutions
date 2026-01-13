use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct Run {
    count: usize,
    value: char,
}

impl Run {
    fn new(count: usize, value: char) -> Self {
        Self { count, value }
    }

    fn expand(&self) -> String {
        self.value.to_string().repeat(self.count)
    }
}

impl From<char> for Run {
    fn from(value: char) -> Self {
        Self { count: 1, value }
    }
}

impl Display for Run {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.count > 1 {
            write!(f, "{}", self.count)?;
        }
        write!(f, "{}", self.value)
    }
}

pub fn encode(source: &str) -> String {
    source
        .chars()
        .fold(Vec::<Run>::new(), |mut acc, c| {
            match acc.last_mut() {
                Some(last) if last.value == c => last.count += 1,
                _ => acc.push(Run::from(c)),
            }
            acc
        })
        .into_iter()
        .map(|run| run.to_string())
        .collect()
}

pub fn decode(source: &str) -> String {
    let mut count_acc = String::new();
    let mut result = String::new();

    for c in source.chars() {
        if c.is_numeric() {
            count_acc.push(c);
        } else {
            let count = count_acc.parse::<usize>().unwrap_or(1);
            result.push_str(&Run::new(count, c).expand());
            count_acc.clear();
        }
    }
    result
}
