// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use bevy::app::{App, Plugin, Startup, Update};
use bevy_mod_outline::OutlinePlugin;

use bevy::prelude::*;
use url::Url;

use crate::{stdin::StdinMessageReceiver, *};

/// Plugin input
///
/// The plugin can be represented as URL:
///
/// * File: file://my_file.µcad?symbol=my::mod::MyPart#L11
/// * Stdin: stdin://
#[derive(Clone)]
pub enum MicrocadPluginInput {
    /// Load and watch an input file.
    File {
        path: std::path::PathBuf,

        /// Full name of resolved symbol to displayed, `std::geo2d::Rect`.
        ///
        /// If there is no symbol given, the source file will be used.
        symbol: Option<String>,

        /// Line number, starting with 1.
        line: Option<u32>,

        /// Time stamp of last modification.
        last_modified: Arc<Mutex<Option<SystemTime>>>,
    },
    /// Remove-controlled via stdin.
    Stdin(Option<StdinMessageReceiver>),
}

impl MicrocadPluginInput {
    /// Construct input from URL.
    pub fn from_url(url: Url) -> anyhow::Result<Self> {
        match url.scheme() {
            "file" => Ok(MicrocadPluginInput::File {
                path: url.to_file_path().expect("Valid file path").to_path_buf(),
                symbol: url.query_pairs().find_map(|(key, value)| {
                    if key == "symbol" {
                        Some(value.into())
                    } else {
                        None
                    }
                }),
                line: url
                    .fragment()
                    .and_then(|frag| frag.strip_prefix('L'))
                    .and_then(|n| n.parse::<u32>().ok()),
                last_modified: Default::default(),
            }),
            "stdin" => Ok(Self::Stdin(None)),
            scheme => Err(anyhow::anyhow!("{scheme} not supported!")),
        }
    }

    /// Get the URL for the input.
    pub fn get_url(&self) -> Url {
        match self {
            MicrocadPluginInput::File {
                path, symbol, line, ..
            } => {
                // Start with base: file://<path>
                let mut url = Url::parse("file://").unwrap();

                // PathBuf -> string (handle both relative and absolute)
                // Note: url::Url requires forward slashes
                let path_str = path.to_string_lossy().replace('\\', "/");
                url.set_path(&path_str);

                // Add symbol as query parameter if present
                if let Some(symbol) = symbol {
                    url.set_query(Some(&format!("symbol={}", symbol)));
                }

                // Add line as fragment (#L<number>) if present
                if let Some(line) = line {
                    url.set_fragment(Some(&format!("L{}", line)));
                }

                url
            }

            MicrocadPluginInput::Stdin(_) => {
                // Simplest possible representation
                Url::parse("stdin://").unwrap()
            }
        }
    }

    fn display_url_human_readable(&self) -> String {
        // Decode the path
        let url = self.get_url();
        let path = percent_encoding::percent_decode_str(url.path()).decode_utf8_lossy();

        // Rebuild the visible parts
        let mut s = format!("{}://{}", url.scheme(), path);

        if let Some(q) = url.query() {
            s.push('?');
            s.push_str(q);
        }
        if let Some(frag) = url.fragment() {
            s.push('#');
            s.push_str(frag);
        }

        s
    }
}

impl std::fmt::Display for MicrocadPluginInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_url_human_readable())
    }
}

pub struct MicrocadPlugin {
    pub input: Option<MicrocadPluginInput>,
    pub config: Config,
}

impl Plugin for MicrocadPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<state::ModelViewState>()
            .add_event::<state::StateEvent>()
            .insert_resource(ClearColor(self.config.theme.primary.to_bevy()))
            .insert_resource(State::new(self.input.clone(), self.config.clone()))
            .add_plugins((OutlinePlugin, MeshPickingPlugin))
            .add_plugins(processor::ProcessorPlugin)
            .add_plugins(material::MaterialPlugin)
            .add_plugins(scene::ScenePlugin)
            .add_systems(Startup, apply_window_settings)
            .add_systems(Update, stdin::handle_stdin_messages)
            .add_systems(Update, state::handle_state_event);
    }
}

fn apply_window_settings(state: Res<State>, mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().expect("Some window");
    window.title = format!(
        "µcad{}",
        match &state.input {
            Some(input) => format!(" - {input}"),
            None => String::new(),
        }
    );
    window.window_level = match state.config.stay_on_top {
        true => bevy::window::WindowLevel::AlwaysOnTop,
        false => bevy::window::WindowLevel::Normal,
    };
    window.present_mode = bevy::window::PresentMode::Mailbox;
}
