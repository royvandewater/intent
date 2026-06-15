pub fn extract(source: &str) -> String {
    render(&entries(source), |title, _kind| title.to_string())
}

/// Like [`extract`], but styled with the colors Cucumber uses for its output:
/// `describe` containers in bold, `it`/`test` leaves in green like passing
/// steps.
pub fn extract_colored(source: &str) -> String {
    render(&entries(source), |title, kind| match kind {
        Block::Describe => format!("{BOLD}{title}{RESET}"),
        Block::Test => format!("{GREEN}{title}{RESET}"),
    })
}

const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[derive(Clone, Copy)]
enum Block {
    Describe,
    Test,
}

struct Entry {
    depth: usize,
    kind: Block,
    title: String,
}

fn render(entries: &[Entry], style: impl Fn(&str, Block) -> String) -> String {
    entries
        .iter()
        .map(|entry| {
            format!(
                "{}{}",
                "  ".repeat(entry.depth),
                style(&entry.title, entry.kind)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn entries(source: &str) -> Vec<Entry> {
    let mut depth: usize = 0;
    let mut entries: Vec<Entry> = Vec::new();

    for line in source.lines() {
        if let Some((kind, title)) = block_of_line(line) {
            entries.push(Entry { depth, kind, title });
        }

        let opens = line.matches('{').count();
        let closes = line.matches('}').count();
        depth = (depth + opens).saturating_sub(closes);
    }

    entries
}

fn block_of_line(line: &str) -> Option<(Block, String)> {
    let trimmed = line.trim_start();
    let kind = block_kind(trimmed)?;

    let start = trimmed.find(['\'', '"', '`'])?;
    let quote = trimmed[start..].chars().next()?;
    let rest = &trimmed[start + quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some((kind, rest[..end].to_string()))
}

fn block_kind(trimmed: &str) -> Option<Block> {
    let kinds = [
        ("describe", Block::Describe),
        ("it", Block::Test),
        ("test", Block::Test),
    ];
    kinds.into_iter().find_map(|(keyword, kind)| {
        trimmed
            .strip_prefix(keyword)
            .filter(|rest| rest.starts_with('(') || rest.starts_with('.'))
            .map(|_| kind)
    })
}
