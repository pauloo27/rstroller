/// Truncate a string and add ellipsis if it's longer than `max_length`
///
/// # Output
///
/// The &str itself converted to a String if it's shorter than `max_length`
/// or a truncated version with an ellipsis at the end.
///
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length - 1])
    }
}
