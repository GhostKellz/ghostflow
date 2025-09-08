use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "gflow")]
#[command(about = "GhostFlow CLI - AI orchestration made simple")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new GhostFlow project
    Init {
        /// Project name
        name: Option<String>,
    },
    /// Run a flow locally
    Run {
        /// Path to flow file
        flow: String,
        /// Input data (JSON)
        #[arg(short, long)]
        input: Option<String>,
    },
    /// Validate a flow definition
    Validate {
        /// Path to flow file
        flow: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init { name } => {
            println!("Initializing project: {}", name.unwrap_or_else(|| "ghostflow-project".to_string()));
        }
        Commands::Run { flow, input } => {
            println!("Running flow: {}", flow);
            if let Some(input_data) = input {
                println!("With input: {}", input_data);
            }
        }
        Commands::Validate { flow } => {
            println!("Validating flow: {}", flow);
        }
    }
    
    Ok(())
}