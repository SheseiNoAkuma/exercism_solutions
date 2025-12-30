pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    assert!(KIDS.contains(&student), "unknown student: {student}");
    let position = KIDS.iter().position(|k| *k == student).unwrap() * 2;
    diagram
        .lines()
        .flat_map(|line| line[position..=position + 1].chars().map(Plant::from))
        .map(|p| p.name)
        .collect::<Vec<_>>()
}

struct Plant {
    name: &'static str,
}
impl From<char> for Plant {
    fn from(c: char) -> Self {
        match c {
            'V' => Plant { name: "violets" },
            'R' => Plant { name: "radishes" },
            'C' => Plant { name: "clover" },
            'G' => Plant { name: "grass" },
            other => panic!("unknown plant: {}", other),
        }
    }
}

static KIDS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];
