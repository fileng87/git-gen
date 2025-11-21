mod config;
mod llm_provider;
mod commands;

use config::AppConfig;

use clap::{Parser, Subcommand};
use commands::commit;

#[derive(Parser)]
#[command(
    name = "git-gen",
    author = "ZyraX <oscarcoll.930714@gmail.com>",
    version = "0.1.0",
    about = "Generate git commit messages using AI",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate commit message from staged changes
    Commit {
        /// Apply the commit after generating the message
        #[arg(short, long)]
        apply: bool,

        /// LLM provider to use (openai or gemini)
        #[arg(short, long, value_enum)]
        llm: Option<llm_provider::LlmProvider>,
    },
}

#[tokio::main]
async fn main() {
    // Load configuration before processing commands
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let cli = Cli::parse();

    if let Err(e) = match cli.command {
        Commands::Commit { apply, llm } => commit::run(llm, apply, config).await,
    } {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
