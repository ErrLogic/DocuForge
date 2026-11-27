use std::{env, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    let mut args = env::args_os().skip(1);
    let Some(output) = args.next() else {
        eprintln!("Usage: docuforge-merge OUTPUT.pdf INPUT1.pdf INPUT2.pdf [INPUT.pdf ...]");
        return ExitCode::FAILURE;
    };
    let inputs: Vec<PathBuf> = args.map(PathBuf::from).collect();
    match docuforge_pdf::merge(&inputs, &PathBuf::from(output)) {
        Ok(pages) => {
            println!("Wrote {pages} pages.");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
