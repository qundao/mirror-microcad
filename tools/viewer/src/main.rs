// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use bevy::render::RenderApp;
use bevy::render::batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport};
use bevy::{DefaultPlugins, app::App};
use clap::Parser;

/// µcad cli
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The input URL defines the resource to be displayed.
    ///
    /// Examples:
    /// * `my_file.µcad` Display contents in file.
    /// * `my_file.µcad?symbol=MyPart#L11`: Display some symbol `MyPart` at line 11.
    /// * `stdin://`: Read from stdin.  
    input: Option<String>,

    /// Windows stays on top.
    #[arg(long, default_value = "false", action = clap::ArgAction::SetTrue)]
    stay_on_top: bool,

    /// Paths to search for files.
    ///
    /// By default, `./lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

impl Args {
    /// Return input string as URL.
    fn input_as_url(&self) -> anyhow::Result<Option<Url>> {
        use std::path::*;

        match &self.input {
            Some(input) => {
                if input.starts_with("stdin:") {
                    let url = Url::parse("stdin://")?;
                    return Ok(Some(url));
                }

                // Split fragment first (after '#')
                let (path_and_query, fragment) = match input.split_once('#') {
                    Some((before, frag)) => (before, Some(frag)),
                    None => (input.as_str(), None),
                };

                // Split query next (after '?')
                let (path_part, query) = match path_and_query.split_once('?') {
                    Some((path, q)) => (path, Some(q)),
                    None => (path_and_query, None),
                };

                // Canonicalize the path if relative
                let path = Path::new(path_part);
                let canonical_path: PathBuf = if path.is_absolute() {
                    path.to_path_buf()
                } else {
                    std::env::current_dir()?.join(path).canonicalize()?
                };

                // Convert canonical path to file:// URL
                let mut url = Url::from_file_path(&canonical_path)
                    .map_err(|_| anyhow::anyhow!("Failed to convert path to file URL"))?;

                // Set query and fragment if present
                if let Some(q) = query {
                    url.set_query(Some(q));
                }
                if let Some(f) = fragment {
                    url.set_fragment(Some(f));
                }

                Ok(Some(url))
            }
            None => Ok(None),
        }
    }
}

use microcad_viewer::Config;

use microcad_viewer::MicrocadPlugin;
use url::Url;

fn main() {
    // Initialize env_logger with a default filter level
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info) // Set the default log level
        .init();

    // Parse the command-line args before starting the app
    let args = Args::parse();
    let url = args.input_as_url();

    let mut config = Config {
        search_paths: args.search_paths,
        stay_on_top: args.stay_on_top,
        ..Default::default()
    };

    if config.search_paths.is_empty() {
        config
            .search_paths
            .append(&mut microcad_builtin::dirs::default_search_paths())
    }

    use microcad_viewer::plugin::MicrocadPluginInput;

    let mut app = App::new();
    app.add_plugins(DefaultPlugins).add_plugins(MicrocadPlugin {
        input: url
            .expect("A valid URL")
            .map(|url| MicrocadPluginInput::from_url(url).expect("Valid URL")),
        config,
    });

    // Workaround for flickering entity bug on Intel GPUs:
    // https://github.com/bevyengine/bevy/issues/18904
    app.sub_app_mut(RenderApp)
        .insert_resource(GpuPreprocessingSupport {
            max_supported_mode: GpuPreprocessingMode::None,
        });

    app.run();
}
