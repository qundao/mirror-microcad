// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad inspector

#![allow(missing_docs)]

use clap::Parser;

use microcad_lang::resolve::{FullyQualify, Symbol, SymbolInfo};
use microcad_lang::src_ref::{Refer, SrcRef, SrcReferrer};
use microcad_lang::syntax::*;

use crossbeam::channel::Sender;
use microcad_viewer_ipc::{ViewerProcessInterface, ViewerRequest};
use std::sync::{Arc, RwLock};
use std::thread;
mod watcher;

use slint::VecModel;

use crate::to_slint::hash_to_shared_string;
use crate::watcher::Watcher;

slint::include_modules!();

mod to_slint;

#[derive(Parser)]
struct Args {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Paths to search for files.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

/// A request to the view model.
#[derive(Debug)]
pub enum ViewModelRequest {
    /// Set source code string.
    SetSourceCode {
        /// Source code to be displayed.
        code: String,
        /// Source file hash.
        hash: u64,
    },
    /// Set the symbol tree items.
    SetSymbolTree(Vec<SymbolTreeModelItem>),
    /// Set the model tree items.
    SetModelTree(Vec<ModelTreeModelItem>),
}

struct Inspector {
    args: Args,

    pub watcher: Watcher,

    pub viewer_process: Arc<RwLock<Option<microcad_viewer_ipc::ViewerProcessInterface>>>,
}

impl Inspector {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            args: Args::parse(),
            watcher: Watcher::new()?,
            viewer_process: Arc::new(RwLock::new(None)),
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        // Create the Slint UI component
        let main_window = MainWindow::new()?;

        let weak = main_window.as_weak();
        let input = self.args.input.clone();
        let (tx, rx): (Sender<ViewModelRequest>, _) = crossbeam::channel::unbounded();
        let search_paths = self.args.search_paths.clone();
        let search_path = search_paths
            .first()
            .unwrap_or(&std::path::PathBuf::from("./lib"))
            .clone(); // HACK

        // Run file watcher thread.
        std::thread::spawn(move || -> anyhow::Result<()> {
            loop {
                // Watch all dependencies of the most recent compilation.
                self.watcher.update(vec![self.args.input.clone()])?;

                let source_file = SourceFile::load(&self.args.input)?;

                match std::fs::read_to_string(&self.args.input) {
                    Ok(code) => tx
                        .send(ViewModelRequest::SetSourceCode {
                            code,
                            hash: source_file.hash,
                        })
                        .expect("No error"),
                    Err(err) => log::error!("{err}"),
                };

                // resolve the file
                let resolve_context = microcad_lang::resolve::ResolveContext::create(
                    source_file,
                    &self.args.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    microcad_lang::diag::DiagHandler::default(),
                )?;

                tx.send(ViewModelRequest::SetSymbolTree({
                    let mut items = Vec::new();

                    resolve_context
                        .symbol_table()
                        .iter()
                        .for_each(|(_, symbol)| {
                            use crate::to_slint::ItemsFromTree;
                            items.append(&mut SymbolTreeModelItem::items_from_tree(symbol))
                        });
                    items
                }))?;

                let mut eval_context = microcad_lang::eval::EvalContext::new(
                    resolve_context,
                    microcad_lang::eval::Stdout::new(),
                    microcad_builtin::builtin_exporters(),
                    microcad_builtin::builtin_importers(),
                );

                if let Some(model) = eval_context
                    .eval()
                    .map_err(|err| anyhow::anyhow!("Eval error: {err}"))?
                {
                    use crate::to_slint::ItemsFromTree;
                    tx.send(ViewModelRequest::SetModelTree(
                        ModelTreeModelItem::items_from_tree(&model),
                    ))?;
                }

                // Wait until anything relevant happens.
                self.watcher.wait()?;
            }
        });

        thread::spawn(move || {
            loop {
                if let Ok(request) = rx.recv() {
                    weak.upgrade_in_event_loop(move |main_window| match request {
                        ViewModelRequest::SetSourceCode { code, hash } => {
                            let items = to_slint::split_source_code(&code);
                            main_window.set_source_code_model(to_slint::model_rc_from_items(items));

                            main_window.set_state(VM_State {
                                current_source_hash: hash_to_shared_string(hash),
                            });
                            main_window.set_source_code(code.into());
                        }
                        ViewModelRequest::SetSymbolTree(items) => {
                            main_window.set_symbol_tree(to_slint::model_rc_from_items(items))
                        }
                        ViewModelRequest::SetModelTree(items) => {
                            main_window.set_model_tree(to_slint::model_rc_from_items(items))
                        }
                    })
                    .expect("No error");
                }
            }
        });

        let viewer_process = self.viewer_process.clone();
        main_window.on_button_launch_viewer_clicked(move || {
            match viewer_process.write() {
                Ok(mut process) => {
                    *process = Some(ViewerProcessInterface::run(&search_path));
                    log::warn!("Already running!");
                }
                Err(err) => log::error!("{err}"),
            };
        });

        let weak = main_window.as_weak();
        let viewer_process = self.viewer_process.clone();
        main_window.on_button_send_source_code_clicked(move || {
            let code = weak
                .upgrade()
                .expect("MainWindow component")
                .get_source_code()
                .to_string();
            log::info!("Send source code: {code}");

            match viewer_process.read() {
                Ok(process) => match &*process {
                    Some(process) => {
                        log::info!("Viewer request");
                        process
                            .send_request(ViewerRequest::SourceCode {
                                path: Some(input.clone()),
                                name: None,
                                code,
                            })
                            .expect("No error");
                    }
                    None => {
                        log::error!("Process is not running!");
                    }
                },
                Err(err) => log::error!("{err}"),
            }
        });

        main_window.run()?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    Inspector::new()?.run()
}
