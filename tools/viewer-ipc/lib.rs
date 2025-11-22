// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Viewer IPC interface

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CursorPosition {
    line: usize,
    col: usize,
}

/// A request sent to the viewers stdin
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ViewerRequest {
    /// Show source code from file.
    ShowSourceCodeFromFile { path: PathBuf },
    /// Show a source code snipped.
    ShowSourceCode {
        path: Option<PathBuf>,
        name: Option<String>,
        code: String,
    },
    /// Set the current cursor range.
    SetCursorRange {
        begin: Option<CursorPosition>,
        end: Option<CursorPosition>,
    },
    /// Hide window
    Show,
    /// Hide window
    Hide,
    /// Exit viewer process.
    Exit,
}

/// A response sent from the viewers stdout
#[derive(Clone, Serialize, Deserialize)]
pub enum ViewerResponse {
    /// The viewer sent a status message.
    StatusMessage(String),
}

/// Our interface to the viewer process.
#[derive(Debug)]
pub struct ViewerProcessInterface {
    request_sender: crossbeam::channel::Sender<ViewerRequest>,
    _response_receiver: crossbeam::channel::Receiver<ViewerResponse>,
}

impl ViewerProcessInterface {
    /// Send a request to the viewer process.
    pub fn send_request(&self, request: ViewerRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    /// Run the viewer process.
    pub fn run(search_paths: &[std::path::PathBuf], show_window: bool) -> Self {
        let search_paths = search_paths
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<String>>();

        log::info!(
            "Running viewer process with search path {}",
            search_paths.join(", ")
        );

        use crossbeam::channel::*;

        let (tx, rx): (Sender<ViewerRequest>, Receiver<ViewerRequest>) = unbounded();
        let (_, resp_rx) = unbounded::<ViewerResponse>();

        std::thread::spawn(move || {
            // Spawn slave process
            let mut command = std::process::Command::new(
                std::env::var("MICROCAD_VIEWER_BIN").unwrap_or("microcad-viewer".to_string()),
            );
            // handle multiple search paths
            search_paths.iter().for_each(|search_path| {
                command.arg("-P").arg(search_path);
            });
            if !show_window {
                command.arg("--hidden");
            }
            let mut child = command
                .arg("stdin://") // run the viewer as slave via stdin.
                .current_dir(std::env::current_dir().expect("current dir"))
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to start slave process");

            let mut stdin = child.stdin.take().expect("stdin");
            let stdout = child.stdout.take().expect("stdout");

            // Thread to read responses
            std::thread::spawn(move || {
                use std::io::BufRead;
                let reader = std::io::BufReader::new(stdout);
                for line in reader.lines().map_while(Result::ok) {
                    log::info!("Response: {line}");
                    // TODO: Read viewer responses.
                    // if let Ok(resp) = serde_json::from_str::<ViewerResponse>(&line) {
                    //     resp_tx.send(resp).unwrap();
                    // }
                }
            });

            // Thread to write requests
            std::thread::spawn(move || loop {
                for req in &rx {
                    use std::io::Write;
                    match serde_json::to_string(&req) {
                        Ok(json) => {
                            log::debug!("Write request as json: {json}");
                            writeln!(stdin, "{}", json).expect("io error");
                            stdin.flush().expect("io error");
                        }
                        Err(_) => todo!(),
                    };
                }
            });

            child.wait().expect("No timeout");
        });

        Self {
            request_sender: tx,
            _response_receiver: resp_rx,
        }
    }
}
