#![allow(dead_code, unused_imports)]

use anyhow::Result;
use clap::Parser;

use smith::cli::{Cli, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Init {
            non_interactive,
            profile,
            team_name,
            org,
            repo,
            project,
            github_project_board,
            bridge,
            skip_github,
            workzone,
            credentials_file,
        } => {
            if non_interactive {
                smith::commands::init::run_non_interactive(
                    profile, team_name, org, repo, project,
                    github_project_board, bridge, skip_github,
                    workzone, credentials_file,
                )?;
            } else {
                smith::commands::init::run()?;
            }
        }
        Command::Install { id } => {
            smith::commands::install::run(&id)?;
        }
        _ => {
            eprintln!("Command not yet ported to smith");
        }
    }

    Ok(())
}
