// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! microcad Viewer processor.

mod model_info;
mod registry;
mod request;
mod systems;

use crate::{processor::registry::InstanceRegistry, to_bevy::ToBevyMesh, *};
pub use processor::model_info::ModelInfo;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::uuid::Uuid,
    render::mesh::Mesh,
};

pub use request::ProcessorRequest;

use crossbeam::channel::{Receiver, Sender};
use microcad_core::RenderResolution;
use microcad_lang::{model::Model, rc::RcMut, render::*, syntax::SourceFile};

/// A processor response.
///
/// Contains the geometry to rendered.
pub enum ProcessorResponse {
    RemoveModelInstances(Vec<Uuid>),
    NewMeshAsset(Uuid, Mesh),
    NewModelInfo(Uuid, ModelInfo),
    UpdateMaterials(Vec<Uuid>),
    SpawnModelInstances(Vec<Uuid>),
}

/// The state of the processor.
pub struct ProcessorContext {
    /// Flag to tell whether to initialize.
    initialized: bool,

    /// Search paths are set during initialization.
    search_paths: Vec<std::path::PathBuf>,

    /// The current render resolutions.
    resolution: microcad_core::RenderResolution,
    theme: config::Theme,

    line_number: Option<u32>,

    pub source_file: Option<std::rc::Rc<SourceFile>>,

    pub model: Option<Model>,

    pub instance_registry: InstanceRegistry,

    /// µcad Render cache.
    pub render_cache: RcMut<RenderCache>,
}

impl Default for ProcessorContext {
    fn default() -> Self {
        Self {
            initialized: false,
            search_paths: Default::default(),
            resolution: Default::default(),
            theme: Default::default(),
            source_file: None,
            model: None,
            line_number: None,
            instance_registry: Default::default(),
            render_cache: RcMut::new(RenderCache::new()),
        }
    }
}

/// The processor  responsible for generating view commands.
///
/// The processor itself runs in a separate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponding responses.
struct Processor {
    /// The state of the processor.
    pub state: ProcessorContext,

    /// Requests.
    pub request_receiver: Receiver<ProcessorRequest>,

    /// Output responses.
    pub response_sender: Sender<ProcessorResponse>,
}

impl Processor {
    /// Handle processor request.
    pub(crate) fn handle_request(
        &mut self,
        request: ProcessorRequest,
    ) -> anyhow::Result<Vec<ProcessorResponse>> {
        match request {
            ProcessorRequest::Initialize { config } => {
                self.state.search_paths = config.search_paths.clone();
                self.state.theme = config.theme;
                self.state.initialized = true;
                Ok(vec![])
            }
            ProcessorRequest::ParseFile(path) => match SourceFile::load(&path) {
                Ok(source_file) => {
                    self.state.source_file = Some(source_file);
                    self.eval()?;
                    self.render(None)?;
                    self.respond()
                }
                Err(err) => {
                    log::error!("{err}");
                    Ok(vec![])
                }
            },
            ProcessorRequest::ParseSource { path, name, source } => {
                match SourceFile::load_from_str(
                    Some(name.unwrap_or(String::from("<none>")).as_str()),
                    path.unwrap_or(std::path::PathBuf::from("<virtual>")),
                    &source,
                ) {
                    Ok(source_file) => {
                        self.state.source_file = Some(source_file);
                        self.eval()?;
                        self.render(None)?;
                        self.respond()
                    }
                    Err(err) => {
                        log::error!("{err}");
                        Ok(vec![])
                    }
                }
            }
            ProcessorRequest::Eval => {
                self.eval()?;
                self.render(None)?;
                self.respond()
            }
            ProcessorRequest::Render(resolution) => {
                self.render(resolution)?;
                self.respond()
            }
            ProcessorRequest::Export { .. } => todo!(),
            ProcessorRequest::SetLineNumber(line_number) => {
                self.state.line_number = line_number;
                self.respond()
            }
        }
    }

    /// We can render if the processor is initialized and we have evaluated some source into a model.
    pub(crate) fn can_render(&self) -> bool {
        self.state.initialized && self.state.model.is_some()
    }

    pub(crate) fn eval(&mut self) -> anyhow::Result<()> {
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
                Ok(())
            }
            None => Err(anyhow::anyhow!("No source code to evaluate.")),
        }
    }

    /// Render geometry from model.
    fn render(&mut self, resolution: Option<RenderResolution>) -> anyhow::Result<()> {
        if self.can_render() {
            let resolution = match resolution {
                Some(resolution) => resolution,
                None => self.state.resolution.clone(),
            };
            let model = self.state.model.as_ref().expect("Model");

            let mut render_context = RenderContext::init(
                model,
                resolution.clone(),
                Some(self.state.render_cache.clone()),
            )?;

            let _: Model = model.render_with_context(&mut render_context)?;

            // Remove unused cache items.
            {
                log::info!("Render cache");
                let mut cache = self.state.render_cache.borrow_mut();
                cache.garbage_collection();
            }

            self.state.resolution = resolution;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Could not render model."))
        }
    }

    fn respond(&mut self) -> anyhow::Result<Vec<ProcessorResponse>> {
        if let Some(model) = self.state.model.clone() {
            let mut responses = Vec::new();
            responses.push(ProcessorResponse::RemoveModelInstances(
                self.state.instance_registry.fetch_model_uuids(),
            ));

            self.state.instance_registry.clear_model_uuids();
            self.generate_responses(&model, &mut responses);
            log::info!("{} responses", responses.len());

            responses.push(ProcessorResponse::UpdateMaterials(
                self.state.instance_registry.fetch_model_uuids(),
            ));

            responses.push(ProcessorResponse::SpawnModelInstances(
                self.state.instance_registry.fetch_model_uuids(),
            ));

            Ok(responses)
        } else {
            Err(anyhow::anyhow!("No model to draw."))
        }
    }

    /// Generate mesh geometry output for model.
    fn generate_responses(&mut self, model: &Model, responses: &mut Vec<ProcessorResponse>) {
        use microcad_lang::model::Element::*;
        match model.render_output_type() {
            microcad_lang::model::OutputType::Geometry2D
            | microcad_lang::model::OutputType::Geometry3D => {}
            microcad_lang::model::OutputType::NotDetermined
            | microcad_lang::model::OutputType::InvalidMixed => return,
        }

        let model_ = model.borrow();
        // We only consider output geometries of workpieces and ignore the rest.
        let recurse = match model_.element() {
            InputPlaceholder | Multiplicity | Group => true,
            Workpiece(_) | BuiltinWorkpiece(_) => {
                let uuid = registry::generate_model_geometry_output_uuid(model);
                let output = model_.output();
                let mut recurse = false;

                // Add a new mesh asset, when we do not have geometry with a uuid in the cache.
                if !self.state.instance_registry.contains_geometry_output(&uuid) {
                    let mesh = match &output.geometry {
                        Some(GeometryOutput::Geometry2D(geometry)) => {
                            Some(geometry.inner.to_bevy_mesh_default())
                        }
                        Some(GeometryOutput::Geometry3D(geometry)) => {
                            Some(geometry.inner.to_bevy_mesh(30.0))
                        }
                        None => None,
                    };

                    match mesh {
                        Some(mesh) => {
                            self.state.instance_registry.insert_geometry_output(uuid);
                            responses.push(ProcessorResponse::NewMeshAsset(uuid, mesh));
                        }
                        None => {
                            recurse = true;
                        }
                    }
                }

                let uuid = registry::generate_model_uuid(model);
                if !self.state.instance_registry.contains_model(&uuid) {
                    self.state.instance_registry.insert_model(uuid);

                    responses.push(ProcessorResponse::NewModelInfo(
                        uuid,
                        ModelInfo::from_model(model),
                    ));
                }

                recurse
            }
        };

        if recurse {
            model_
                .children()
                .for_each(|model| self.generate_responses(model, responses));
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
                state: ProcessorContext::default(),
                request_receiver,
                response_sender,
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
        app.add_systems(Startup, systems::initialize_processor)
            .add_systems(Update, systems::handle_processor_responses)
            .add_systems(Update, systems::file_reload)
            .add_systems(Update, systems::handle_pick_event);
    }
}
