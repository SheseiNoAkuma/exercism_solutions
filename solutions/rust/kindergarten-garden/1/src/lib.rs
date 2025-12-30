pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    assert!(KIDS.contains(&student), "unknown student: {}", student);
    let kid_index = KIDS.iter().position(|k| *k == student).unwrap() * 2;
    diagram
        .lines()
        .map(|line| line.chars().map(Plant::from).collect::<Vec<_>>())
        .flat_map(|r| {
            r[kid_index..kid_index + 2]
                .iter()
                .map(|p| p.name)
                .collect::<Vec<_>>()
        })
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
