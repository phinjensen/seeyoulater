/// Takes a string in plural form and a count, and returns the &str without the final s. If a word
/// that doesn't pluralize with a single s is used, this will have to change to return a String.
pub fn singular_plural(word: &str, count: isize) -> &str {
    if count == 1 {
        &word[..word.len() - 1]
    } else {
        word
    }
}
