// based on
// https://github.com/GNOME/glib/blob/811b4c90add48c640b4271c227ac17b015fed141/glib/gmarkup.c#L2149
pub fn escape_gtk_markup(input: &str) -> String {
    let mut result = String::new();
    let pending = input.chars().clone();

    for c in pending {
        result.push_str(
            match c {
                '&' => Some("&amp;"),
                '<' => Some("&lt;"),
                '>' => Some("&gt;"),
                '\'' => Some("&apos;"),
                '"' => Some("&quot;"),
                _ => None,
            }
            .unwrap_or(&c.to_string()),
        );
    }

    result
}
