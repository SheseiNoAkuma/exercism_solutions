const DIVIDERS: &[u32] = &[3, 5, 7];

pub fn raindrops(n: u32) -> String {
    let sound = DIVIDERS
        .iter()
        .filter_map(|&d| if n % d == 0 { Some(to_sound(d)) } else { None })
        .collect::<String>();

    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}

fn to_sound(divider: u32) -> &'static str {
    match divider {
        3 => "Pling",
        5 => "Plang",
        7 => "Plong",
        _ => panic!("Invalid divider: {}", divider),
    }
}
