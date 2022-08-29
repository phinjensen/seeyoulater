/// Takes a string in plural form and a count, and returns the &str without the final s. If a word
/// that doesn't pluralize with a single s is used, this will have to change to return a String.
pub fn singular_plural(word: &str, count: isize) -> String {
    if count == 1 {
        match word {
            "these" => String::from("this"),
            _ => word[..word.len() - 1].to_string(),
        }
    } else {
        word.to_string()
    }
}
