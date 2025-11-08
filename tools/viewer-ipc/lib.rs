// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    SourceCodeFromFile {
        path: PathBuf,
    },
    SourceCode {
        path: Option<PathBuf>,
        name: Option<String>,
        code: String,
    },
    CursorRange {
        begin: Option<CursorPosition>,
        end: Option<CursorPosition>,
    },
    /// Exit viewer process.
    Exit,
}

/// A response sent from the viewers stdout
#[derive(Clone, Serialize, Deserialize)]
pub enum ViewerResponse {
    /// The viewer sent a status message.
    StatusMessage(String),
}

#[derive(Default)]
pub struct ViewerState {}

pub struct ViewerProcessInterface {
    _state: ViewerState,
    request_sender: crossbeam::channel::Sender<ViewerRequest>,
    _response_receiver: crossbeam::channel::Receiver<ViewerResponse>,
}

impl ViewerProcessInterface {
    pub fn send_request(&self, request: ViewerRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    pub fn run(std_search_path: impl AsRef<std::path::Path>) -> Self {
        log::info!(
            "Run viewer process with search path {}",
            std_search_path.as_ref().display()
        );
        use crossbeam::channel::*;

        let (tx, rx): (Sender<ViewerRequest>, Receiver<ViewerRequest>) = unbounded();
        let (_, resp_rx) = unbounded::<ViewerResponse>();
        let std_search_path = std_search_path.as_ref().to_path_buf();

        std::thread::spawn(move || {
            // Spawn slave process
            let mut child = std::process::Command::new(
                std::env::var("MICROCAD_VIEWER_BIN").unwrap_or("microcad-viewer".to_string()),
            )
            .arg("stdin") // run the slave binary
            .arg("-P")
            .arg(std_search_path.to_str().unwrap())
            .current_dir(std::env::current_dir().unwrap())
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start slave process");

            let mut stdin = child.stdin.take().unwrap();
            let stdout = child.stdout.take().unwrap();

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
            std::thread::spawn(move || {
                loop {
                    for req in &rx {
                        use std::io::Write;
                        let json = serde_json::to_string(&req).unwrap();
                        log::debug!("Write request as json: {json}");
                        writeln!(stdin, "{}", json).unwrap();
                        stdin.flush().unwrap();
                    }
                }
            });

            child.wait().expect("No timeout");
        });

        Self {
            _state: Default::default(),
            request_sender: tx,
            _response_receiver: resp_rx,
        }
    }
}
