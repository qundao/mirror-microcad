// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad inspector

#![allow(missing_docs)]

use clap::Parser;

use microcad_lang::resolve::{FullyQualify, Symbol};
use microcad_lang::syntax::*;

use std::sync::{Arc, Mutex, mpsc};
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

struct Inspector {
    args: Args,

    pub watcher: Watcher,
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
        items.push(Self {
            depth: depth as i32,
            name: symbol.full_name().to_string().into(),
        });

        symbol.with_children(|(_, symbol)| {
            Self::_from_tree(symbol, items, depth + 1);
        })
    }
}

impl Inspector {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            args: Args::parse(),
            watcher: Watcher::new()?,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        // Create the Slint UI component
        let main_window = MainWindow::new()?;

        let weak = main_window.as_weak();
        let input = self.args.input.clone();

        // Run file watcher thread.
        std::thread::spawn(move || -> anyhow::Result<()> {
            loop {
                let (tx, rx): (mpsc::Sender<Vec<ModelTreeModelItem>>, _) = mpsc::channel();
                // Watch all dependencies of the most recent compilation.
                self.watcher.update(vec![self.args.input.clone()])?;

                let source_file = SourceFile::load(&self.args.input)?;

                // resolve the file
                let resolve_context = microcad_lang::resolve::ResolveContext::create(
                    source_file,
                    &self.args.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    microcad_lang::diag::DiagHandler::default(),
                )?;

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
                    let items = ModelTreeModelItem::items_from_tree(&model);

                    // Wait until anything relevant happens.
                    tx.send(items)?;

                    weak.upgrade_in_event_loop(move |main_window| {
                        let items = rx.recv().expect("No error");
                        main_window.set_model_tree(model_rc_from_items(items))
                    })?;
                }

                self.watcher.wait()?;
            }
        });

        main_window.on_button_launch_3d_view_clicked(move || {
            // let main_window = weak.unwrap();

            // Run process thread
            // Shared handle to the child's stdin so we can send messages
            let child_stdin: Arc<Mutex<Option<std::process::ChildStdin>>> =
                Arc::new(Mutex::new(None));

            let stdin_clone = Arc::clone(&child_stdin);
            let mut input = input.clone();

            // Spawn the thread to launch and manage the child process
            std::thread::spawn(move || -> anyhow::Result<()> {
                input.set_extension("stl");
                let input = std::env::current_dir().expect("Current dir").join(input);

                log::info!("Input {}", input.display());

                let mut child = std::process::Command::new(
                    "/home/micha/Work/mcad/bevy_stdin/target/debug/bevy_stdin",
                ) // Replace with your long-lived process
                .arg(input)
                .stdin(std::process::Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");

                // Share the child's stdin with the main thread
                *stdin_clone.lock().expect("Successful lock") = child.stdin.take();

                // Wait for the process to exit (this will block)
                let status = child.wait()?;
                println!("Child exited with: {status}");
                Ok(())
            });
        });

        main_window.run()?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    Inspector::new()?.run()
}
