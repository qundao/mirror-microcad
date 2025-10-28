// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{any, path::PathBuf};

use serde::{Deserialize, Serialize};

/// A request sent to the viewers stdin
#[derive(Clone, Serialize, Deserialize)]
pub enum ViewerRequest {
    SourceCodeFromFile { path: PathBuf },
    CursorPosition { line: usize, col: usize },
}

/// A response sent from the viewers stdout
#[derive(Clone, Serialize, Deserialize)]
pub enum ViewerResponse {
    /// The viewer sent a status message.
    StatusMessage(String),
}

pub enum ViewerState {
    SourceFile {
        path: PathBuf,
        line: usize,
        col: usize,
    },
}

pub struct ViewerProcessInterface {
    state: ViewerState,
    request_sender: crossbeam::channel::Sender<ViewerRequest>,
    response_receiver: crossbeam::channel::Receiver<ViewerResponse>,
}

impl ViewerProcessInterface {
    pub fn send_request(&self, request: ViewerRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    pub fn run(&self) {
        /*// Run process thread
        // Shared handle to the child's stdin so we can send messages
        let child_stdin: std::sync::Arc<Mutex<Option<std::process::ChildStdin>>> =
            Arc::new(Mutex::new(None));

        let stdin_clone = Arc::clone(&child_stdin);
        let mut input = input.clone();

        // Spawn the thread to launch and manage the child process
        std::thread::spawn(move || -> anyhow::Result<()> {
            log::info!("Input {}", input.display());

            let mut child = std::process::Command::new(self.viewer_executable)
                .arg("--stdin")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");

            // Share the child's stdin with the main thread
            *stdin_clone.lock().expect("Successful lock") = child.stdin.take();

            // Wait for the process to exit (this will block)
            let status = child.wait()?;
            println!("Child exited with: {status}");
            Ok(())
        });*/
    }
}
