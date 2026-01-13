fn say_chunk(sentence: &mut String, n: u64) {
    match n {
        0..=15 => {
            let word = match n {
                0 => "zero",
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven",
                8 => "eight",
                9 => "nine",
                10 => "ten",
                11 => "eleven",
                12 => "twelve",
                13 => "thirteen",
                14 => "fourteen",
                15 => "fifteen",
                _ => unreachable!(),
            };
            sentence.push_str(word);
        }
        16..=19 => {
            say_chunk(sentence, n - 10);
            if n - 10 == 8 {
                assert!(sentence.pop() == Some('t'));
            }
            sentence.push_str("teen");
        }
        20..=99 => {
            let tens = match n / 10 {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => unreachable!(),
            };
            sentence.push_str(tens);
            let ones = n % 10;
            if ones != 0 {
                sentence.push('-');
                say_chunk(sentence, ones);
            }
        }
        _ => unreachable!(),
    }
}

fn say_whole(sentence: &mut String, n: u64) {
    let (name, cutoff) = match n {
        0..=99 => {
            say_chunk(sentence, n);
            return;
        }
        100..=999 => ("hundred", 10_u64.pow(2)),
        1_000..=999_999 => ("thousand", 10_u64.pow(3)),
        1_000_000..=999_999_999 => ("million", 10_u64.pow(6)),
        1_000_000_000..=999_999_999_999 => ("billion", 10_u64.pow(9)),
        1_000_000_000_000..=999_999_999_999_999 => ("trillion", 10_u64.pow(12)),
        1_000_000_000_000_000..=999_999_999_999_999_999 => ("quadrillion", 10_u64.pow(15)),
        1_000_000_000_000_000_000..=u64::MAX => ("quintillion", 10_u64.pow(18)),
    };
    let hi = n / cutoff;
    let lo = n % cutoff;
    say_whole(sentence, hi);
    sentence.push(' ');
    sentence.push_str(name);
    if lo != 0 {
        sentence.push(' ');
        say_whole(sentence, lo);
    }
}

pub fn encode(n: u64) -> String {
    let mut sentence = String::new();
    say_whole(&mut sentence, n);
    sentence
}