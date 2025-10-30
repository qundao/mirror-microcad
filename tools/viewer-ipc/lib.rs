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
    state: ViewerState,
    request_sender: crossbeam::channel::Sender<ViewerRequest>,
    response_receiver: crossbeam::channel::Receiver<ViewerResponse>,
}

impl ViewerProcessInterface {
    pub fn send_request(&self, request: ViewerRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    pub fn run() -> Self {
        use crossbeam::channel::*;

        let (tx, rx): (Sender<ViewerRequest>, Receiver<ViewerRequest>) = unbounded();

        // Spawn slave process
        let mut child = std::process::Command::new(
            std::env::var("MICROCAD_VIEWER_BIN").unwrap_or("microcad-viewer".to_string()),
        )
        .arg("--stdin") // run the slave binary
        .current_dir("/home/micha/Work/mcad/mcad/tools/viewer")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start slave process");

        let mut stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let (resp_tx, resp_rx) = unbounded::<ViewerResponse>();

        // Thread to read responses
        std::thread::spawn(move || {
            use std::io::BufRead;
            let reader = std::io::BufReader::new(stdout);
            for line in reader.lines().map_while(Result::ok) {
                log::info!("Response: {line}");
                //                if let Ok(resp) = serde_json::from_str::<ViewerResponse>(&line) {
                //                  resp_tx.send(resp).unwrap();
                //            }
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

        Self {
            state: Default::default(),
            request_sender: tx,
            response_receiver: resp_rx,
        }
    }
}
