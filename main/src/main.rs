use cmt_cli::{cli, CliError, HiArgs};
use std::error::Error;
use std::process::ExitCode;

fn main() {
    let matches = cli().try_get_matches().unwrap_or_else(|e| e.exit());
    let subcommand = matches.subcommand().unwrap();
    match subcommand {
        ("grep", matches) => match HiArgs::from(matches) {
            Ok(arg) => match search(&arg) {
                Ok(_) => {}
                Err(_) => {}
            },
            Err(error) => {
                println!("{error}")
            }
        },
        _ => unreachable!(),
    }
}

/// The top-level entry point for single-threaded search.
///
/// This recursively steps through the file list (current directory by default)
/// and searches each file sequentially.
fn search(arg: &HiArgs) -> Result<bool, Box<dyn Error>> {
    println!("Start searching with params: {:?}", arg);
    Ok(true)
}
