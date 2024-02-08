pub mod input;
pub mod init;
pub mod list;
pub mod new;
pub mod edit;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "iris", about = "Bash script creation and management CLI.")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Init,
    New {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    Edit {
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
}

pub async fn call() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => list::list_scripts()?,
        Commands::Init => init::init()?,
        Commands::New { name, description } => new::new_script(name, description).await?,
        Commands::Edit { name, description } => edit::edit_script(name, description).await?,
    }

    Ok(())
}
