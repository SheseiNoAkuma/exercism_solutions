use std::fmt::Display;

pub fn encode(source: &str) -> String {
    let encoded = source
        .chars()
        .fold(Vec::new(), |mut acc: Vec<EncodedChar>, c| {
            match acc.last_mut() {
                Some(last) if last.is_same(c) => last.increment(),
                _ => acc.push(EncodedChar::with_char(c)),
            }
            acc
        });

    encoded
        .into_iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
}

pub fn decode(source: &str) -> String {
    let (chain, _) = source
        .chars()
        .fold((Vec::new(), None), |(mut acc, mut count), c| match c {
            _ if !c.is_numeric() => {
                acc.push(EncodedChar::new(count.unwrap_or(1), c));
                (acc, None)
            }
            _ if c.is_numeric() => {
                count = count
                    .map(|n| n * 10)
                    .and_then(|n| c.to_digit(10).map(|x| x as u8 + n))
                    .or_else(|| c.to_digit(10).map(|x| x as u8));
                (acc, count)
            }
            _ => unreachable!("char not supported: {}", c),
        });

    chain
        .into_iter()
        .map(|c| c.to_original())
        .collect::<String>()
}

#[derive(Debug)]
struct EncodedChar {
    count: u8,
    char: char,
}
impl EncodedChar {
    fn new(count: u8, char: char) -> Self {
        Self { count, char }
    }

    fn with_char(char: char) -> Self {
        Self { count: 1, char }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn is_same(&self, other: char) -> bool {
        self.char == other
    }

    fn to_original(&self) -> String {
        (0..self.count).map(|_| self.char).collect::<String>()
    }
}
impl Display for EncodedChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.count == 1 {
            write!(f, "{}", self.char)
        } else {
            write!(f, "{}{}", self.count, self.char)
        }
    }
}
