const ZERO: &str =
    "    ____\u{20}
   / __ \\
  / / / /
 / /_/ /\u{20}
\\____/  ";

fn wrap(s: &str) -> String {
    let mut out = String::new();
    let mut word = String::new();
    let mut len = 0;

    for c in s.chars() {
        if c.is_whitespace() {
            len += word.len() + 1;
            out.extend(word.drain(..));
            out.push(c);
        } else {
            word.push(c);
        }

        if len > 70 {
            out.push('\n');
            len = 0;
        }
    }

    out.extend(word.chars());

    out
}

fn center_lines(s: &str) -> String {
    s.lines().map(|l| format!("|{:^78}|", l)).collect::<Vec<_>>().join("\n")
}

pub fn draw(s: &str) -> String {
    format!(
        r#"
+------------------------------------------------------------------------------+
|                                                                              |
|{:^78}|
|                                                                              |
{}
|                                                                              |
|{:^78}|
|                                                                              |
{}
|                                                                              |
+------------------------------------------------------------------------------+
"#,
        "It has been",
        center_lines(ZERO),
        "days",
        center_lines(&wrap(&format!("since {}", s)))
    )
}
