use std::ops::Add;

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    ((start_bottles - take_down + 1)..=start_bottles)
        .rev()
        .fold("".to_string(), |acc, i| acc.add(format_verse(i).as_str()))
}

fn format_verse(i: u32) -> String {
    format!(
        "{0} green {1} hanging on the wall,\n{0} green {1} hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {2} green {3} hanging on the wall.\n\n",
        number_to_word(i as u8),
        bottles_or_bottle(i),
        number_to_word((i - 1) as u8).to_ascii_lowercase(),
        bottles_or_bottle(i - 1)
    )
}

fn bottles_or_bottle(n: u32) -> String {
    match n {
        1 => "bottle".to_string(),
        _ => "bottles".to_string(),
    }
}

fn number_to_word(n: u8) -> &'static str {
    match n {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        1 => "One",
        0 => "No",
        n => unimplemented!("{n} bottles are unimplemented"),
    }
}
