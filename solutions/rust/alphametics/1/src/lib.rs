use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words = input.split(char::is_whitespace)
        .map(|string: &str| string.chars().filter(|c| c.is_alphabetic()).collect::<String>() )
        .filter(|string| !string.is_empty() )
        .collect::<Vec<String>>();
    let mut unique_chars: HashSet<char> = HashSet::new();
    for word in &words {
        for c in word.chars() {
            unique_chars.insert(c);
        }
    }

    solve_r(&words, &mut unique_chars.iter().collect(), &mut (HashMap::new(), HashMap::new()))
}

fn solve_r (words: &Vec<String>, remaining_chars: &mut String, bindings: &mut (HashMap<char, u8>, HashMap<u8, char>)) -> Option<HashMap<char, u8>> {
    if remaining_chars.len() == 0 {
        let mut calculation: u64 = 0;
        let mut last = 0;
        for word in words {
            let mut num = 0;
            for (i, c) in word.chars().rev().enumerate() {
                num += *bindings.0.get(&c).unwrap() as u64 * 10u64.pow(i as u32) as u64;
            }
            last = num;
            calculation += last;
        }

        calculation -= last;
        if calculation == last {
            return Some(bindings.0.clone())
        } else {
            return None
        }
    }

    let c = remaining_chars.pop().unwrap();
    for i in 0..=9 as u8 {
        if bindings.1.get(&i).is_some() {
            continue
        } else if i == 0 && words.iter().fold(false, |leading, word| leading || word.chars().next().unwrap() == c) {
            continue
        }

        bindings.0.insert(c, i);
        bindings.1.insert(i, c);

        let result = solve_r(words, remaining_chars, bindings);
        if result.is_some() {
            return result;
        }

        bindings.1.remove_entry(&i);
    }

    remaining_chars.push(c);

    None
}