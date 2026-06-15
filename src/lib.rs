pub fn extract(source: &str) -> String {
    source
        .lines()
        .filter_map(title_of_line)
        .collect::<Vec<_>>()
        .join("\n")
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
}
