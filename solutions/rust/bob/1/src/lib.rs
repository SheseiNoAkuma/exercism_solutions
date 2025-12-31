pub fn reply(message: &str) -> &str {
    match (message.is_question(), message.is_yelling()) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Sure.",
        (_, true) => "Whoa, chill out!",
        _ if message.is_silence() => "Fine. Be that way!",
        _ => "Whatever.",
    }
}

trait StringExt {
    fn is_question(&self) -> bool;
    fn is_yelling(&self) -> bool;
    fn is_silence(&self) -> bool;
}
impl StringExt for &str {
    fn is_question(&self) -> bool {
        self.trim().ends_with("?")
    }

    fn is_yelling(&self) -> bool {
        self.trim().chars().filter(|c| c.is_alphabetic()).count() > 0
            && self
                .trim()
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    }

    fn is_silence(&self) -> bool {
        self.trim().is_empty()
    }
}
