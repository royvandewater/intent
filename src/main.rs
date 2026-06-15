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

    println!("{}", intent::extract(&source));
    ExitCode::SUCCESS
}
