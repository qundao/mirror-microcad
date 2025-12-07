// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad viewer

use bevy::{
    app::App,
    render::{
        batching::gpu_preprocessing::{GpuPreprocessingMode, GpuPreprocessingSupport},
        RenderApp,
    },
    DefaultPlugins,
};
use clap::Parser;
use miette::IntoDiagnostic;

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

    /// Windows stays hidden (and can be shown via IPC.
    #[arg(long, default_value = "false", action = clap::ArgAction::SetTrue)]
    hidden: bool,

    /// Verbosity level (use -v, -vv, or -vvv)
    #[arg(short, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,

    /// Paths to search for files.
    ///
    /// By default, `./std/lib` (if it exists) and `~/.microcad/lib` are used.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

impl Args {
    /// Return input string as URL.
    fn input_as_url(&self) -> miette::Result<Option<Url>> {
        use std::path::*;

        match &self.input {
            Some(input) => {
                let (scheme, part) = input.split_once("://").unwrap_or(("file", input));
                match scheme {
                    "stdin" => {
                        let url = Url::parse("stdin://").into_diagnostic()?;
                        Ok(Some(url))
                    }
                    "file" => {
                        // Split fragment first (after '#')
                        let (path_and_query, fragment) = match part.split_once('#') {
                            Some((before, frag)) => (before, Some(frag)),
                            None => (part, None),
                        };

                        // Split query next (after '?')
                        let (path_part, query) = match path_and_query.split_once('?') {
                            Some((path, q)) => (path, Some(q)),
                            None => (path_and_query, None),
                        };

                        // Canonicalize the path if relative
                        let path = microcad_lang::resolve::microcad_file_path(path_part)
                            .map_err(|err| anyhow::anyhow!("{err}"))?;

                        let canonical_path: PathBuf = if path.is_absolute() {
                            path.to_path_buf()
                        } else {
                            let path = std::env::current_dir().into_diagnostic()?.join(path_part);
                            log::info!("Path: {path:?}");
                            path.canonicalize().into_diagnostic()?
                        };

                        // Convert canonical path to file:// URL
                        let mut url = Url::from_file_path(&canonical_path)
                            .map_err(|_| miette::miette!("Failed to convert path to file URL"))?;

                        // Set query and fragment if present
                        if let Some(q) = query {
                            url.set_query(Some(q));
                        }
                        if let Some(f) = fragment {
                            url.set_fragment(Some(f));
                        }

                        Ok(Some(url))
                    }
                    scheme => Err(miette::miette!("Unknown scheme: {scheme}")),
                }
            }
            None => Ok(None),
        }
    }
}

use microcad_viewer::Config;

use microcad_viewer::MicrocadPlugin;
use url::Url;

fn main() {
    // Parse the command-line args before starting the app
    let args = Args::parse();

    // Initialize env_logger with a default filter level
    env_logger::Builder::from_default_env()
        .filter_level(match args.verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            3 => log::LevelFilter::Trace,
            _ => panic!("unknown verbosity level"),
        }) // Set the default log level
        .init();

    let url = args.input_as_url();
    let url = match &url {
        Ok(url) => url.clone(),
        Err(err) => {
            log::error!(
                "{err} ({input})",
                input = match &args.input {
                    Some(input) => format!("({input})"),
                    None => String::new(),
                }
            );
            None
        }
    };

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
    app
        // Power-saving reactive rendering for applications.
        .insert_resource(bevy::winit::WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugins(MicrocadPlugin {
            input: url.map(|url| MicrocadPluginInput::from_url(url).expect("Valid URL")),
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
