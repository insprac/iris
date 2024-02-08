pub fn trim_markdown_code_block(s: &str) -> String {
    let mut lines = s.lines();
    let mut result = String::new();

    if let Some(line) = lines.next() {
        if line.starts_with("```") {
            for line in lines {
                if line.starts_with("```") {
                    return result;
                }
                result.push_str(line);
                result.push('\n');
            }
        }
    }

    s.to_string()
}

pub fn trim_quotes(s: &str) -> String {
    if s.starts_with('"') && s.ends_with('"') {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}
