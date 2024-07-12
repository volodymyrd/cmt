use clap::{crate_description, crate_name, Arg, ArgAction, ArgMatches, Command};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliError {
    message: String,
}

impl CliError {
    fn new(msg: &str) -> Self {
        CliError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// The collection of paths we want to search for.
///
/// This guarantees that there is always at least one path.
#[derive(Debug)]
struct Paths {
    /// The actual paths.
    paths: Vec<PathBuf>,
}

/// The disjunction of patterns to search for.
///
/// The number of patterns can be empty, e.g., via `-f /dev/null`.
#[derive(Debug)]
struct Patterns {
    /// The actual patterns to match.
    patterns: Vec<String>,
}

/// A high level representation of CLI arguments.
#[derive(Debug)]
pub struct HiArgs {
    paths: Paths,
    patterns: Patterns,
}

impl HiArgs {
    pub fn from(arg: &ArgMatches) -> Result<HiArgs, CliError> {
        let phrase = arg
            .get_one::<String>("phrase")
            .ok_or(CliError::new("No phrase provided!"))?;

        let path = arg
            .get_one::<String>("path")
            .ok_or(CliError::new("No path provided!"))?;

        Ok(Self {
            paths: Paths {
                paths: vec![PathBuf::from(path)],
            },
            patterns: Patterns {
                patterns: vec![phrase.to_string()],
            },
        })
    }
}

pub fn cli() -> Command {
    clap::command!(crate_name!())
        .about(crate_description!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            clap::command!("grep")
                .about("grep is a search tool that recursively searches the current directory")
                .disable_version_flag(true)
                .arg(
                    Arg::new("phrase")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .help("Searching phrase"),
                )
                .arg(
                    Arg::new("path")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .help("Searching path"),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
}
