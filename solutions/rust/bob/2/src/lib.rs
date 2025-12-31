const FINE: &str = "Fine. Be that way!";
const CALM: &str = "Calm down, I know what I'm doing!";
const SURE: &str = "Sure.";
const WHOA: &str = "Whoa, chill out!";
const WHATEVER: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match (message.is_question(), message.is_yelling()) {
        (true, true) => CALM,
        (true, _) => SURE,
        (_, true) => WHOA,
        _ if message.is_silence() => FINE,
        _ => WHATEVER,
    }
}

trait StringExt {
    fn is_question(&self) -> bool;
    fn is_yelling(&self) -> bool;
    fn is_silence(&self) -> bool;
}
impl StringExt for &str {
    fn is_question(&self) -> bool {
        self.ends_with("?")
    }

    fn is_yelling(&self) -> bool {
        self.chars().any(|c| c.is_alphabetic()) && *self == self.to_uppercase()
    }

    fn is_silence(&self) -> bool {
        self.is_empty()
    }
}
