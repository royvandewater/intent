use std::io::IsTerminal;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: intent <test-file>");
        return ExitCode::FAILURE;
    };

    let source = match std::fs::read_to_string(&path) {
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

/// Color only when writing to a terminal and `NO_COLOR` is unset, so piped or
/// redirected output stays plain text.
fn use_color() -> bool {
    std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
}
