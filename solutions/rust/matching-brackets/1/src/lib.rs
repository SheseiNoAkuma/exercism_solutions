pub fn brackets_are_balanced(string: &str) -> bool {
    string
        .chars()
        .fold(Vec::new(), |mut acc: Vec<char>, c| match c {
            '(' | '{' | '[' => {
                acc.push(c);
                acc
            }
            _ if is_closing(c, acc.last()) => {
                acc.pop();
                acc
            }
            ')' | ']' | '}' => {
                acc.push(c);
                acc
            }
            _ => acc,
        })
        .is_empty()
}

fn is_closing(c: char, last: Option<&char>) -> bool {
    match (c, last) {
        (')', Some('(')) => true,
        (']', Some('[')) => true,
        ('}', Some('{')) => true,
        _ => false,
    }
}
