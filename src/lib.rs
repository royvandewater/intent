pub fn extract(source: &str) -> String {
    let mut depth: usize = 0;
    let mut lines: Vec<String> = Vec::new();

    for line in source.lines() {
        if let Some(title) = title_of_line(line) {
            lines.push(format!("{}{}", "  ".repeat(depth), title));
        }

        let opens = line.matches('{').count();
        let closes = line.matches('}').count();
        depth = (depth + opens).saturating_sub(closes);
    }

    lines.join("\n")
}

const BLOCK_KEYWORDS: [&str; 3] = ["describe", "it", "test"];

fn title_of_line(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if !starts_block(trimmed) {
        return None;
    }

    let start = trimmed.find(['\'', '"', '`'])?;
    let quote = trimmed[start..].chars().next()?;
    let rest = &trimmed[start + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(rest[..end].to_string())
}

fn starts_block(trimmed: &str) -> bool {
    BLOCK_KEYWORDS.iter().any(|keyword| {
        trimmed
            .strip_prefix(keyword)
            .is_some_and(|rest| rest.starts_with('(') || rest.starts_with('.'))
    })
}
