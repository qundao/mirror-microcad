// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer Standard input interface.

use std::{io::BufRead, time::Duration};

use crossbeam::channel::Receiver;

use bevy::ecs::{resource::Resource, system::Res};

use microcad_viewer_ipc::ViewerRequest;

use crate::processor::ProcessorRequest;

#[derive(Resource)]
pub struct StdinMessageReceiver(Receiver<ViewerRequest>);

impl StdinMessageReceiver {
    pub fn run() -> Self {
        // Create channel for stdin reader to communicate with Bevy
        let (sender, receiver) = crossbeam::channel::unbounded();

        // Spawn thread to read from stdin
        std::thread::spawn(move || {
            let stdin = std::io::stdin();

            loop {
                for line in stdin.lock().lines().map_while(Result::ok) {
                    match serde_json::from_str::<ViewerRequest>(&line) {
                        Ok(msg) => {
                            if sender.send(msg).is_err() {
                                break;
                            }
                        }
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
    receiver: Res<StdinMessageReceiver>,
    mut event_writer: bevy::prelude::EventWriter<ProcessorRequest>,
) {
    for viewer_request in receiver.0.try_iter() {
        match viewer_request {
            microcad_viewer_ipc::ViewerRequest::SourceCodeFromFile { path } => {
                event_writer.write(ProcessorRequest::ParseFile(path));
            }
            microcad_viewer_ipc::ViewerRequest::SourceCode { path, name, code } => {
                event_writer.write(ProcessorRequest::ParseCode { path, name, code });
            }
            microcad_viewer_ipc::ViewerRequest::CursorRange { .. } => todo!(),
        }
    }
}
