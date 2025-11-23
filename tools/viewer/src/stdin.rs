// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer Standard input interface.

use std::{io::BufRead, time::Duration};

use bevy::ecs::system::Query;
use bevy::window::Window;
use crossbeam::channel::Receiver;

use bevy::ecs::resource::Resource;
use bevy::prelude::{AppExit, EventWriter};

use microcad_viewer_ipc::ViewerRequest;

use crate::plugin::MicrocadPluginInput;
use crate::processor::ProcessorRequest;

#[derive(Resource, Clone)]
pub struct StdinMessageReceiver(Receiver<ViewerRequest>);

impl StdinMessageReceiver {
    pub fn run() -> Self {
        log::info!("Run stdin message receiver");
        // Create channel for stdin reader to communicate with Bevy
        let (sender, receiver) = crossbeam::channel::unbounded();

        // Spawn thread to read from stdin
        std::thread::spawn(move || {
            let stdin = std::io::stdin();

            loop {
                for line in stdin.lock().lines().map_while(Result::ok) {
                    log::info!("Line: {line:?}");
                    match serde_json::from_str::<ViewerRequest>(&line) {
                        Ok(msg) => match sender.send(msg) {
                            Ok(_) => {
                                log::info!("Message sent!");
                            }
                            Err(err) => {
                                log::error!("{err}");
                            }
                        },
                        Err(e) => eprintln!("Invalid input: {e}"),
                    }
                }

                std::thread::sleep(Duration::from_millis(20));
            }
        });

        Self(receiver)
    }
}

/// Process stdin messages into processor requests.
pub fn handle_stdin_messages(
    state: bevy::prelude::ResMut<crate::State>,
    mut exit: EventWriter<AppExit>,
    mut windows: Query<&mut Window>,
) {
    if let Some(MicrocadPluginInput::Stdin(Some(stdin))) = &state.input {
        for viewer_request in stdin.0.try_iter() {
            log::info!("{viewer_request:?}");

            use microcad_viewer_ipc::ViewerRequest::*;
            match viewer_request {
                ShowSourceCodeFromFile { path } => {
                    state
                        .processor
                        .send_request(ProcessorRequest::ParseFile(path))
                        .expect("No error");
                }
                ShowSourceCode { path, name, code } => {
                    state
                        .processor
                        .send_request(ProcessorRequest::ParseSource {
                            path,
                            name,
                            source: code,
                        })
                        .expect("No error");
                }
                SetCursorRange { .. } => todo!(),
                Exit => {
                    exit.write(AppExit::Success);
                }
                ViewerRequest::Show => {
                    let mut window = windows.single_mut().expect("A window");
                    window.visible = true;
                }
                ViewerRequest::Hide => {
                    let mut window = windows.single_mut().expect("A window");
                    window.visible = false;
                }
            }
        }
    }
}
