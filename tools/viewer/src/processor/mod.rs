// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer processor.

mod geometry_output;
mod mesh_instance;
mod systems;

use bevy::{
    app::{Plugin, Startup, Update},
    ecs::event::Event,
};
pub use geometry_output::*;

use crossbeam::channel::{Receiver, Sender};
use microcad_core::RenderResolution;
use microcad_lang::{model::Model, rc::RcMut, render::*, syntax::SourceFile};

/// A processor request.
///
/// Commands that can be passed to the [`Processor`].
#[derive(Event, Clone)]
pub enum ProcessorRequest {
    /// Initialize the interpreter.
    ///
    /// Request must only be sent once and sets the initialize flag to `true`.
    Initialize {
        search_paths: Vec<std::path::PathBuf>,
    },
    /// Parse file.
    ParseFile(std::path::PathBuf),
    /// Parse some code into a SourceFile.
    ParseSource {
        /// Virtual file path
        path: Option<std::path::PathBuf>,
        /// Optional name of the source code snippet, e.g. the full file name.
        name: Option<String>,
        /// The actual source code.
        source: String,
    },
    /// Evaluate source file into a model to be rendered.
    Eval,
    /// Set cursor position
    /*SetCursorRange {
        begin: Option<Position>,
        end: Option<Position>,
    },*/

    /// Render the geometry. This message should be sent when the source code has been modified.
    Render(Option<microcad_core::RenderResolution>),
    /// Export the geometry to a file.
    Export {
        /// File name.
        filename: std::path::PathBuf,
        /// Optional exporter ("svg", "stl").
        exporter: Option<String>,
    },
}

/// A processor response.
///
/// Contains the geometry to rendered.
pub enum ProcessorResponse {
    /// The response contains output geometry from a render request.
    OutputGeometry(Vec<OutputGeometry>),
}

/// The state of the interpreter.

#[derive(Default)]
pub struct ProcessorState {
    /// Flag to tell whether to initializer.
    initialized: bool,

    /// Search paths are set during initialization.
    search_paths: Vec<std::path::PathBuf>,

    resolution: microcad_core::RenderResolution,

    pub source_file: Option<std::rc::Rc<SourceFile>>,
    pub model: Option<Model>,
}

/// The processor  responsable for generating view commands.
///
/// The processor itself runs in a seperate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponing responses.
struct Processor {
    /// The state of the processor.
    pub state: ProcessorState,

    /// Requests.
    pub request_receiver: Receiver<ProcessorRequest>,

    /// Output responses.
    pub response_sender: Sender<ProcessorResponse>,

    /// Render cache.
    pub render_cache: RcMut<RenderCache>,
}

#[derive(thiserror::Error, Debug)]
pub enum PipelineError {
    /// Input/output error.
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    /// Parse error.
    #[error("Parse error: {0}")]
    ParseError(#[from] microcad_lang::parse::ParseError),
}

pub type PipelineResult<T> = Result<T, PipelineError>;

/// A processing pipeline.
pub trait Pipeline {
    /// Initialize the pipeline with search paths.
    fn initialize(&mut self, additional_search_paths: Vec<std::path::PathBuf>);

    /// Parse a file.
    fn parse_file(
        &mut self,
        path: std::path::PathBuf,
    ) -> Result<std::rc::Rc<SourceFile>, PipelineError> {
        Ok(SourceFile::load(path)?)
    }

    fn eval(&mut self) -> PipelineResult<()>;

    fn render(&mut self, resolution: Option<RenderResolution>) -> PipelineResult<()>;
}

impl Processor {
    /// Handle processor request.
    pub(crate) fn handle_request(
        &mut self,
        request: ProcessorRequest,
    ) -> anyhow::Result<Vec<ProcessorResponse>> {
        match request {
            ProcessorRequest::Initialize { search_paths } => {
                self.state.search_paths = search_paths.clone();
                self.state.initialized = true;
                Ok(vec![])
            }
            ProcessorRequest::ParseFile(path) => {
                self.state.source_file = SourceFile::load(&path).ok();
                self.eval()?;
                self.render(None)
            }
            ProcessorRequest::ParseSource { path, name, source } => {
                self.state.source_file = SourceFile::load_from_str(
                    name.unwrap_or(String::from("<none>")).as_str(),
                    path.unwrap_or(std::path::PathBuf::from("<virtual>")),
                    &source,
                )
                .ok();
                self.eval()?;
                self.render(None)
            }
            ProcessorRequest::Eval => {
                self.eval()?;
                self.render(None)
            }
            ProcessorRequest::Render(resolution) => self.render(resolution),
            ProcessorRequest::Export { .. } => todo!(),
        }
    }

    /// We can render if the processor is initialized and we have evaluated some source into a model.
    pub(crate) fn can_render(&self) -> bool {
        self.state.initialized && self.state.model.is_some()
    }

    pub(crate) fn eval(&mut self) -> anyhow::Result<Vec<ProcessorResponse>> {
        match &self.state.source_file {
            Some(source_file) => {
                // resolve the file
                let resolve_context = microcad_lang::resolve::ResolveContext::create(
                    source_file.clone(),
                    &self.state.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    microcad_lang::diag::DiagHandler::default(),
                )?;

                let mut eval_context = microcad_lang::eval::EvalContext::new(
                    resolve_context,
                    microcad_lang::eval::Stdout::new(),
                    microcad_builtin::builtin_exporters(),
                    microcad_builtin::builtin_importers(),
                );

                self.state.model = eval_context.eval()?;

                Ok(vec![])
            }
            None => Err(anyhow::anyhow!("No source code to evaluate.")),
        }
    }

    /// Render geometry from µcad file.
    pub(crate) fn render(
        &mut self,
        resolution: Option<RenderResolution>,
    ) -> anyhow::Result<Vec<ProcessorResponse>> {
        if self.can_render() {
            let resolution = match resolution {
                Some(resolution) => resolution,
                None => self.state.resolution.clone(),
            };
            let model = self.state.model.as_ref().expect("Model");

            let mut render_context =
                RenderContext::init(model, resolution.clone(), Some(self.render_cache.clone()))?;
            let model: Model = model.render_with_context(&mut render_context)?;

            // Remove unused cache items.
            {
                log::info!("Render cache");
                let mut cache = self.render_cache.borrow_mut();
                cache.garbage_collection();
            }

            let mut mesh_geometry = Vec::new();
            self.state.resolution = resolution;
            Self::generate_mesh_geometry_from_model(&model, &mut mesh_geometry);
            Ok(vec![ProcessorResponse::OutputGeometry(mesh_geometry)])
        } else {
            Err(anyhow::anyhow!("Could not render model."))
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
                state: ProcessorState::default(),
                request_receiver,
                response_sender,
                render_cache: RcMut::new(RenderCache::new()),
            };

            loop {
                if let Ok(request) = processor.request_receiver.recv()
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
            .add_systems(Update, systems::handle_processor_responses)
            .add_systems(Update, systems::handle_external_reload);
    }
}
