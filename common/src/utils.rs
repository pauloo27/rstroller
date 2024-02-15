pub fn truncate_string(s: &str, max_length: usize) -> &str {
    if s.len() > max_length {
        &s[..max_length]
    } else {
        s
    }
}
