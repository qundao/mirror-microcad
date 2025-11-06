// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad inspector

#![allow(missing_docs)]

use clap::Parser;

use microcad_lang::resolve::{FullyQualify, Symbol};
use microcad_lang::syntax::*;

use crossbeam::channel::Sender;
use microcad_viewer_ipc::{ViewerProcessInterface, ViewerRequest};
use std::sync::{Arc, RwLock};
use std::thread;
mod watcher;

use slint::VecModel;

use crate::watcher::Watcher;

slint::include_modules!();

#[derive(Parser)]
struct Args {
    /// Input µcad file.
    pub input: std::path::PathBuf,

    /// Paths to search for files.
    #[arg(short = 'P', long = "search-path", action = clap::ArgAction::Append, default_value = "./lib", global = true)]
    pub search_paths: Vec<std::path::PathBuf>,
}

trait ItemsFromTree<T, D = usize>: Sized
where
    D: Default,
{
    fn _from_tree(tree: &T, items: &mut Vec<Self>, depth: D);

    /// Create all items from model, including children
    fn items_from_tree(tree: &T) -> Vec<Self> {
        let mut items = Vec::new();
        Self::_from_tree(tree, &mut items, D::default());
        items
    }
}

/// Create ModelRc from items.
fn model_rc_from_items<T: Sized + Clone + 'static>(items: Vec<T>) -> slint::ModelRc<T> {
    slint::ModelRc::new(VecModel::from(items))
}

use microcad_lang::model::Model;

impl ItemsFromTree<Model> for ModelTreeModelItem {
    fn _from_tree(model: &Model, items: &mut Vec<Self>, depth: usize) {
        let model_ = model.borrow();
        let creator = match model_.element.creator() {
            Some(creator) => VM_Creator {
                symbol_name: creator.symbol.full_name().to_string().into(),
            },
            None => VM_Creator::default(),
        };

        items.push(Self {
            depth: depth as i32,
            element: model_.element.value.to_string().into(),
            src_ref: model_.element.src_ref.to_string().into(),
            creator,
        });
        model_
            .children()
            .for_each(|model| Self::_from_tree(model, items, depth + 1))
    }
}

impl ItemsFromTree<Symbol> for SymbolTreeModelItem {
    fn _from_tree(symbol: &Symbol, items: &mut Vec<Self>, depth: usize) {
        use microcad_lang::src_ref::SrcReferrer;

        items.push(Self {
            depth: depth as i32,
            name: symbol.full_name().to_string().into(),
            source_hash: symbol.src_ref().source_hash() as i32,
        });

        symbol.with_children(|(_, symbol)| {
            Self::_from_tree(symbol, items, depth + 1);
        })
    }
}

fn split_source_code(source: &str) -> Vec<SourceCodeModelItem> {
    let mut items = Vec::new();
    let mut byte_index = 0;

    for (line_number, line) in source.split_inclusive('\n').enumerate() {
        // `split_inclusive('\n')` keeps the newline at the end of each line,
        // which helps preserve correct byte ranges and offsets.
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();

        items.push(SourceCodeModelItem {
            line: line.to_string().into(),
            line_number: line_number as i32,
            byte_range_start: byte_index as i32,
            byte_range_end: (byte_index + line_len) as i32,
            ..Default::default()
        });

        byte_index += line_len;
    }

    // Handle case where the last line does not end with a newline
    if !source.ends_with('\n') && !source.is_empty() {
        if let Some(last_line) = source.lines().last() {
            let line_len = last_line.len();
            let line_start = source.len() - line_len;

            items.push(SourceCodeModelItem {
                line: last_line.to_string().into(),
                line_number: items.len() as i32,
                byte_range_start: line_start as i32,
                byte_range_end: source.len() as i32,
                ..Default::default()
            });
        }
    }

    items
}

/// A request to the view model.
#[derive(Debug)]
pub enum ViewModelRequest {
    /// Set source code string.
    SetSourceCode(String),
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

                match std::fs::read_to_string(&self.args.input) {
                    Ok(code) => tx
                        .send(ViewModelRequest::SetSourceCode(code))
                        .expect("No error"),
                    Err(err) => log::error!("{err}"),
                };

                let source_file = SourceFile::load(&self.args.input)?;

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
                        ViewModelRequest::SetSourceCode(source_code) => {
                            let items = split_source_code(&source_code);
                            main_window.set_source_code_model(model_rc_from_items(items));

                            main_window.set_source_code(source_code.into());
                        }
                        ViewModelRequest::SetSymbolTree(items) => {
                            main_window.set_symbol_tree(model_rc_from_items(items))
                        }
                        ViewModelRequest::SetModelTree(items) => {
                            main_window.set_model_tree(model_rc_from_items(items))
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
