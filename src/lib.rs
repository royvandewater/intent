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

fn title_of_line(line: &str) -> Option<String> {
    let start = line.find('\'')?;
    let rest = &line[start + 1..];
    let end = rest.find('\'')?;
    Some(rest[..end].to_string())
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
}
