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

/// Diffs the extracted intent of two versions of a test file. Titles present
/// only in `new_source` are shown as additions (green `+`), titles present only
/// in `old_source` as removals (red `-`), and shared titles as plain context.
/// Returns an empty string when the intent is identical.
pub fn diff_intent(old_source: &str, new_source: &str, colored: bool) -> String {
    let old = extract(old_source);
    let new = extract(new_source);
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();

    let edits = diff_lines(&old_lines, &new_lines);
    if edits.iter().all(|(sign, _)| *sign == Sign::Context) {
        return String::new();
    }

    edits
        .iter()
        .map(|(sign, line)| render_edit(*sign, line, colored))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Whether a changed `path` should be shown given the filename `filters`. With
/// no filters every path matches; otherwise a path matches when it equals a
/// filter or ends with `/<filter>`, so a bare basename works from any directory.
pub fn path_matches(path: &str, filters: &[String]) -> bool {
    if filters.is_empty() {
        return true;
    }
    filters
        .iter()
        .any(|filter| path == filter || path.ends_with(&format!("/{filter}")))
}

#[derive(Clone, Copy, PartialEq)]
enum Sign {
    Added,
    Removed,
    Context,
}

fn render_edit(sign: Sign, line: &str, colored: bool) -> String {
    let body = match sign {
        Sign::Added => format!("+ {line}"),
        Sign::Removed => format!("- {line}"),
        Sign::Context => format!("  {line}"),
    };
    match (colored, sign) {
        (true, Sign::Added) => format!("{GREEN}{body}{RESET}"),
        (true, Sign::Removed) => format!("{RED}{body}{RESET}"),
        _ => body,
    }
}

/// A longest-common-subsequence line diff, ordered so removals precede the
/// additions that replace them.
fn diff_lines<'a>(old: &[&'a str], new: &[&'a str]) -> Vec<(Sign, &'a str)> {
    let (n, m) = (old.len(), new.len());
    let mut lcs = vec![vec![0usize; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            lcs[i][j] = if old[i] == new[j] {
                lcs[i + 1][j + 1] + 1
            } else {
                lcs[i + 1][j].max(lcs[i][j + 1])
            };
        }
    }

    let mut edits = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < n && j < m {
        if old[i] == new[j] {
            edits.push((Sign::Context, old[i]));
            i += 1;
            j += 1;
        } else if lcs[i + 1][j] >= lcs[i][j + 1] {
            edits.push((Sign::Removed, old[i]));
            i += 1;
        } else {
            edits.push((Sign::Added, new[j]));
            j += 1;
        }
    }
    edits.extend(old[i..].iter().map(|line| (Sign::Removed, *line)));
    edits.extend(new[j..].iter().map(|line| (Sign::Added, *line)));
    edits
}

const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
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
