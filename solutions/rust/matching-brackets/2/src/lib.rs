pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .fold(Vec::new(), |mut acc: Vec<char>, c| {
            match c {
                '(' | '{' | '[' => {
                    acc.push(c);
                }
                _ if is_closing_last(c, acc.last()) => {
                    acc.pop();
                }
                ')' | ']' | '}' => {
                    acc.push(c);
                }
                _ => (),
            }
            acc
        })
        .is_empty()
}

fn is_closing_last(c: char, last: Option<&char>) -> bool {
    matches!(
        (c, last),
        (')', Some('(')) | (']', Some('[')) | ('}', Some('{'))
    )
}
