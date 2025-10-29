use bevy::{DefaultPlugins, app::App};
use clap::Parser;

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// File input (optional).
    input: Option<std::path::PathBuf>,

    /// Receive commands via stdin.
    #[arg(long, default_value = "false", action = clap::ArgAction::SetTrue)]
    stdin: bool,

    /// Windows stays on top.
    #[arg(long, default_value = "false", action = clap::ArgAction::SetTrue)]
    stay_on_top: bool,

    /// Paths to search for files.
    ///
    /// By default, `./lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

use microcad_viewer::Config;

use microcad_viewer::MicrocadPlugin;

fn main() {
    // Initialize env_logger with a default filter level
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info) // Set the default log level
        .init();

    // Parse the command-line args before starting the app
    let args = Args::parse();

    let mut config = Config {
        search_paths: args.search_paths,
        stay_on_top: args.stay_on_top,
        ..Default::default()
    };

    if config.search_paths.is_empty() {
        config
            .search_paths
            .append(&mut Config::default_search_paths())
    }

    use microcad_viewer::plugin::MicrocadPluginMode;

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MicrocadPlugin {
            mode: match (args.input, args.stdin) {
                (None, true) => MicrocadPluginMode::Stdin,
                (Some(input), false) => MicrocadPluginMode::InputFile(input),
                _ => MicrocadPluginMode::Empty,
            },
            config,
        })
        .run();
}
