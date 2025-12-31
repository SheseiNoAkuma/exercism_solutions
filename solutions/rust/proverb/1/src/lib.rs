pub fn build_proverb(list: &[&str]) -> String {
    let maybe_first_item = list.first();
    (0..list.len())
        .map(|i| format_line(list[i], list.get(i + 1), maybe_first_item))
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_line(item1: &str, maybe_item2: Option<&&str>, maybe_first: Option<&&str>) -> String {
    maybe_item2
        .map(|item2| format!("For want of a {item1} the {item2} was lost."))
        .unwrap_or_else(|| format!("And all for the want of a {}.", maybe_first.unwrap()))
}
