// based on
// https://github.com/GNOME/glib/blob/811b4c90add48c640b4271c227ac17b015fed141/glib/gmarkup.c#L2149
pub fn escape_gtk_markup(input: &str) -> String {
    let mut result = String::new();
    let mut pending = input.chars().clone();

    while let Some(c) = pending.next() {
        match c {
            '&' => {
                result.push_str("&amp;");
            }
            '<' => {
                result.push_str("&lt;");
            }
            '>' => {
                result.push_str("&gt;");
            }
            '\'' => {
                result.push_str("&apos;");
            }
            '"' => {
                result.push_str("&quot;");
            }
            c => {
                result.push(c);
            }
        }
    }

    result
}
