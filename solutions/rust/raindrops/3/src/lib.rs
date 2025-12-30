const SOUNDS: &[(u32, &str)] = &[(3, "Pling"), (5, "Plang"), (7, "Plong")];

pub fn raindrops(n: u32) -> String {
    let sound: String = SOUNDS
        .iter()
        .filter_map(|(divider, sound)| (n % divider == 0).then_some(sound.to_string()))
        .collect();

    if !sound.is_empty() {
        sound
    } else {
        n.to_string()
    }
}
