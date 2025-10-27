use bevy::{DefaultPlugins, app::App};
use clap::Parser;

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File input.
    input: std::path::PathBuf,

    /// Paths to search for files.
    ///
    /// By default, `./lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

use microcad_viewer::Settings;

use microcad_viewer::MicrocadPlugin;

fn main() {
    // Initialize env_logger with a default filter level
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info) // Set the default log level
        .init();

    // Parse the command-line args before starting the app
    let args = Args::parse();

    let mut settings = Settings {
        search_paths: args.search_paths,
        ..Default::default()
    };

    if settings.search_paths.is_empty() {
        settings
            .search_paths
            .append(&mut Settings::default_search_paths())
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MicrocadPlugin {
            input: args.input,
            settings,
        })
        .run();
}
