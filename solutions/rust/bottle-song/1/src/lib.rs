use std::ops::Add;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let x = ((start_bottles - take_down + 1)..=start_bottles)
        .rev()
        .fold("".to_string(), |acc, i| {
            acc.add(replace_verse(i).as_str()) + "\n\n"
        });
    println!("{}", x);
    x
}

fn replace_verse(i: u32) -> String {
    let mut part1 = VERSE_P1.replace("%CURRENT%", number_to_word(i as u8).unwrap());

    if i == 1 {
        part1 = part1.replace("bottles", "bottle");
    }

    let mut part2 = VERSE_P2.replace(
        "%NEXT%",
        &number_to_word((i - 1) as u8)
            .map(|w| w.to_ascii_lowercase())
            .unwrap(),
    );
    if i == 2 {
        part2 = part2.replace("bottles", "bottle")
    }
    part1 + &part2
}

static VERSE_P1: &str = "%CURRENT% green bottles hanging on the wall,
%CURRENT% green bottles hanging on the wall,
And if one green bottle should accidentally fall,
";

static VERSE_P2: &str = "There'll be %NEXT% green bottles hanging on the wall.";

fn number_to_word(n: u8) -> Option<&'static str> {
    const WORDS: [&str; 11] = [
        "No", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
    ];

    WORDS.get(n as usize).copied()
}
