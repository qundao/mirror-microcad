// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer processor.

mod geometry_output;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::event::Event,
};
pub use geometry_output::*;

use crossbeam::channel::{Receiver, Sender};
use microcad_lang::{model::Model, rc::RcMut, render::*, syntax::SourceFile};

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Event, Clone)]
pub enum ProcessorRequest {
    /// Initialize the interpreter.
    Initialize {
        search_paths: Vec<std::path::PathBuf>,
        path: std::path::PathBuf,
        resolution: microcad_core::RenderResolution,
    },
    /// Render the geometry. This message is sent when the input file has been modified.
    Render,
}

/// An interpreter output.
///
/// Contains the geometry to rendered.
pub enum ProcessorResponse {
    /// The response contains output geometry.
    OutputGeometry(Vec<OutputGeometry>), // SceneBoundsChanged(Bounds3D)
}

/// The state of the interpreter.
pub enum ProcessorState {
    /// The interpreter waits to be initialized.
    Idle,
    /// The interpreter is ready to process render commands.
    Ready {
        search_paths: Vec<std::path::PathBuf>,
        path: std::path::PathBuf,
        resolution: microcad_core::RenderResolution,
    },
}

/// The processor  responsable for generating view commands.
///
/// The processor itself runs in a seperate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponing responses.
struct Processor {
    /// The state of the processor.
    pub state: ProcessorState,
    pub request_handler: Receiver<ProcessorRequest>,

    /// Outputs
    pub response_sender: Sender<ProcessorResponse>,
    // pub cursor_position: SourceLocation,
    /// Render cache.
    pub render_cache: RcMut<RenderCache>,
}

impl Processor {
    /// Handle processor request.
    pub(crate) fn handle_request(
        &mut self,
        request: ProcessorRequest,
    ) -> anyhow::Result<Vec<ProcessorResponse>> {
        match (&self.state, request) {
            (
                ProcessorState::Idle,
                ProcessorRequest::Initialize {
                    search_paths,
                    path,
                    resolution,
                },
            ) => {
                self.state = ProcessorState::Ready {
                    search_paths,
                    path,
                    resolution,
                };
                Ok(vec![])
            }
            (ProcessorState::Ready { .. }, ProcessorRequest::Render) => self.render(),
            _ => Ok(vec![]),
        }
    }

    /// Render geometry from µcad file.
    pub(crate) fn render(&self) -> anyhow::Result<Vec<ProcessorResponse>> {
        match &self.state {
            ProcessorState::Idle => unreachable!("Can only render in Ready state"),
            ProcessorState::Ready {
                search_paths,
                path,
                resolution,
            } => {
                let source_file = SourceFile::load(path)?;

                // resolve the file
                let resolve_context = microcad_lang::resolve::ResolveContext::create(
                    source_file,
                    search_paths,
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
                    use microcad_lang::render::RenderWithContext;

                    let mut render_context = RenderContext::init(
                        &model,
                        resolution.clone(),
                        Some(self.render_cache.clone()),
                    )?;

                    let model: Model = model.render_with_context(&mut render_context)?;

                    // Remove unused cache items.
                    {
                        let mut cache = self.render_cache.borrow_mut();
                        cache.garbage_collection();
                    }

                    let mut mesh_geometry = Vec::new();
                    Self::generate_mesh_geometry_from_model(&model, &mut mesh_geometry);
                    Ok(vec![ProcessorResponse::OutputGeometry(mesh_geometry)])
                } else {
                    Ok(vec![])
                }
            }
        }
    }

    /// Generate mesh geometry output for model.
    fn generate_mesh_geometry_from_model(model: &Model, mesh_geometry: &mut Vec<OutputGeometry>) {
        match OutputGeometry::from_model(model) {
            Some(output_geometry) => {
                mesh_geometry.push(output_geometry);
            }
            None => {
                let model_ = model.borrow();
                model_
                    .children()
                    .for_each(|model| Self::generate_mesh_geometry_from_model(model, mesh_geometry))
            }
        }
    }
}

pub struct ProcessorInterface {
    pub request_sender: Sender<ProcessorRequest>,
    pub response_receiver: Receiver<ProcessorResponse>,
}

impl ProcessorInterface {
    /// Send request.
    pub fn send_request(&self, request: ProcessorRequest) -> anyhow::Result<()> {
        Ok(self.request_sender.send(request)?)
    }

    /// Run the processing thread and create interface.
    pub fn run() -> Self {
        let (request_sender, request_receiver) = crossbeam::channel::unbounded();
        let (response_sender, response_receiver) = crossbeam::channel::unbounded();

        std::thread::spawn(move || {
            let mut processor = Processor {
                state: ProcessorState::Idle,
                request_handler: request_receiver,
                response_sender,
                render_cache: RcMut::new(RenderCache::default()),
            };

            loop {
                if let Ok(request) = processor.request_handler.recv()
                    && let Ok(responses) = processor.handle_request(request)
                {
                    for response in responses {
                        processor.response_sender.send(response).expect("No error");
                    }
                }
            }
        });

        Self {
            request_sender,
            response_receiver,
        }
    }
}

/// The processor bevy plugin.
pub struct ProcessorPlugin;

impl Plugin for ProcessorPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ProcessorRequest>()
            .add_systems(Startup, systems::startup_processor)
            .add_systems(Update, systems::handle_processor_request)
            .add_systems(Update, systems::handle_processor_responses);
    }
}
