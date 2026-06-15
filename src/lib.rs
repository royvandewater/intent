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

const BLOCK_KEYWORDS: [&str; 2] = ["describe", "it"];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_source_produces_empty_output() {
        let output = extract("");

        assert_eq!(output, "");
    }

    #[test]
    fn single_it_block_prints_its_title() {
        let output = extract("it('adds two numbers', () => {})");

        assert_eq!(output, "adds two numbers");
    }

    #[test]
    fn multiple_it_blocks_print_one_title_per_line() {
        let output = extract("it('first', () => {})\nit('second', () => {})");

        assert_eq!(output, "first\nsecond");
    }

    #[test]
    fn it_nested_in_describe_is_indented_under_it() {
        let source = "describe('Calculator', () => {\n  it('adds', () => {})\n})";

        let output = extract(source);

        assert_eq!(output, "Calculator\n  adds");
    }

    #[test]
    fn double_quoted_titles_are_supported() {
        let output = extract("it(\"adds two numbers\", () => {})");

        assert_eq!(output, "adds two numbers");
    }

    #[test]
    fn backtick_titles_are_supported() {
        let output = extract("it(`adds two numbers`, () => {})");

        assert_eq!(output, "adds two numbers");
    }

    #[test]
    fn non_describe_or_it_lines_with_quotes_are_ignored() {
        let source = "describe('Calculator', () => {\n  it('adds', () => {\n    expect(add(1, 2)).toBe('three')\n  })\n})";

        let output = extract(source);

        assert_eq!(output, "Calculator\n  adds");
    }
}
