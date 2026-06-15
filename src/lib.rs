pub fn extract(source: &str) -> String {
    if let Some(start) = source.find('\'') {
        let rest = &source[start + 1..];
        if let Some(end) = rest.find('\'') {
            return rest[..end].to_string();
        }
    }
    String::new()
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
}
