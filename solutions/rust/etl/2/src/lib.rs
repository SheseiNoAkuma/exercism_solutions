use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut target: BTreeMap<char, i32> = BTreeMap::new();

    for (score, letters) in h {
        for letter in letters {
            target.insert(letter.to_ascii_lowercase(), *score);
        }
    }

    target
}
