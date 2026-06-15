use std::io::IsTerminal;
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(first) = args.next() else {
        eprintln!("usage: intent <test-file>\n       intent --diff");
        return ExitCode::FAILURE;
    };

    if first == "--diff" {
        return run_diff();
    }

    run_extract(&first)
}

fn run_extract(path: &str) -> ExitCode {
    let source = match std::fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("intent: cannot read {path}: {error}");
            return ExitCode::FAILURE;
        }
    };

    if use_color() {
        println!("{}", intent::extract_colored(&source));
    } else {
        println!("{}", intent::extract(&source));
    }
    ExitCode::SUCCESS
}

/// Print the intent diff for every test file that changed between `main` and
/// `HEAD`, using two-dot semantics (`main..HEAD`): a straight tip-to-tip
/// comparison, not against the merge base.
fn run_diff() -> ExitCode {
    let changed = match git(&["diff", "--name-only", "main", "HEAD"]) {
        Ok(output) => output,
        Err(error) => {
            eprintln!("intent: {error}");
            return ExitCode::FAILURE;
        }
    };

    let colored = use_color();
    for path in changed.lines() {
        let old = git(&["show", &format!("main:{path}")]).unwrap_or_default();
        let new = git(&["show", &format!("HEAD:{path}")]).unwrap_or_default();
        let diff = intent::diff_intent(&old, &new, colored);
        if diff.is_empty() {
            continue;
        }
        println!("{}", header(path, colored));
        println!("{diff}\n");
    }
    ExitCode::SUCCESS
}

fn header(path: &str, colored: bool) -> String {
    if colored {
        format!("\x1b[1m{path}\x1b[0m")
    } else {
        path.to_string()
    }
}

/// Run a git command, returning its stdout. A missing file (e.g. added or
/// deleted between the two tips) surfaces as an error the caller treats as
/// empty content.
fn git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| format!("failed to run git: {error}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Color only when writing to a terminal and `NO_COLOR` is unset, so piped or
/// redirected output stays plain text.
fn use_color() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
}
