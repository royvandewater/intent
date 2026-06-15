pub fn extract(_source: &str) -> String {
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
}
