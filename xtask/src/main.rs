use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli")]
#[command(author, version, about = "A CLI tool for copyright updates and doc testing", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Update copyright notices across project files
    UpdateCopyright {
        /// Target year to set in the copyright notice
        #[arg(short, long)]
        year: Option<u16>,

        /// Paths to update (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// Run documentation tests
    DocTest {
        /// Filter specific tests by name
        #[arg(short, long)]
        filter: Option<String>,

        /// Run tests without executing actions
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::UpdateCopyright { year, path } => {
            println!("Updating copyright for path: {path}");
            if let Some(y) = year {
                println!("Target year set to: {y}");
            }
            // TODO: Add copyright update logic
        }
        Commands::DocTest { filter, dry_run } => {
            println!("Running doc tests... (dry_run = {dry_run})");
            if let Some(f) = filter {
                println!("Filter applied: {f}");
            }
            // TODO: Add doc test logic
        }
    }
}
