mod sub_command;

use clap::{Parser, Subcommand};
use sub_command::create_new_project;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    sub_command: Option<SubCommand>,
}

#[derive(Subcommand)]
enum SubCommand {
    /// Generate new Svelte UI project with <name> as project name
    New {
        /// Name for new project
        #[clap()]
        project_name: String, 
    }
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.sub_command {
        Some(SubCommand::New { project_name }) => {
            create_new_project(project_name).await?;
        }
        None => {}
    }

    Ok(())
}
