use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut target: BTreeMap<char, i32> = BTreeMap::new();

    h.keys().for_each(|k| {
        h.get(k).unwrap().iter().for_each(|c| {
            for lc in c.to_lowercase() {
                target.insert(lc, *k);
            }
        })
    });

    target
}
