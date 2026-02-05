// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crossbeam::channel::{Receiver, Sender};
use microcad_core::RenderResolution;
use microcad_lang::{
    diag::Diag,
    model::Model,
    render::{GeometryOutput, RenderContext, RenderWithContext},
    syntax::SourceFile,
};

use crate::{
    processor::{
        ModelInfo, ProcessorRequest, ProcessorResponse,
        context::{ProcessingState, ProcessorContext},
    },
    to_bevy::ToBevyMesh,
};

/// The processor is responsible for generating bevy meshes and commands sent to the view.
///
/// The processor itself runs in a separate thread and can be controlled
/// via [`ProcessorInterface`] by sending requests and handling the corresponding responses.
pub struct Processor {
    /// The state of the processor.
    pub context: ProcessorContext,

    /// Requests.
    pub request_receiver: Receiver<ProcessorRequest>,

    /// Output responses.
    pub response_sender: Sender<ProcessorResponse>,
}

impl Processor {
    fn state_change(&mut self, state: ProcessingState) {
        self.context.state = state.clone();
        self.response_sender
            .send(ProcessorResponse::StateChanged(state))
            .expect("No error")
    }

    /// Handle processor request.
    pub(crate) fn handle_request(
        &mut self,
        request: ProcessorRequest,
    ) -> miette::Result<Vec<ProcessorResponse>> {
        match request {
            ProcessorRequest::Initialize { config } => {
                self.context.search_paths = config.search_paths.clone();
                self.context.theme = config.theme;
                self.context.initialized = true;
                self.context.state = ProcessingState::Idle;
                Ok(vec![])
            }
            ProcessorRequest::ParseFile(path) => {
                self.state_change(ProcessingState::Busy(0.0));

                match SourceFile::load(&path) {
                    Ok(source_file) => {
                        self.context.source_file = Some(source_file);
                        self.eval()?;
                        self.render(None)?;
                        self.respond()
                    }
                    Err(err) => {
                        log::error!("{err}");
                        self.state_change(ProcessingState::Error);
                        Ok(vec![])
                    }
                }
            }
            ProcessorRequest::ParseSource { path, name, source } => {
                self.state_change(ProcessingState::Busy(0.0));

                match SourceFile::load_from_str(
                    name.as_deref(),
                    path.unwrap_or(std::path::PathBuf::from("<virtual>")),
                    &source,
                ) {
                    Ok(source_file) => {
                        self.context.source_file = Some(source_file);
                        self.eval()?;
                        self.render(None)?;
                        self.respond()
                    }
                    Err(errors) => {
                        for err in errors {
                            log::error!("{err}");
                        }
                        self.state_change(ProcessingState::Error);
                        Ok(vec![])
                    }
                }
            }
            ProcessorRequest::Eval => {
                self.state_change(ProcessingState::Busy(0.0));
                self.eval()?;
                self.render(None)?;
                self.respond()
            }
            ProcessorRequest::Render(resolution) => {
                self.state_change(ProcessingState::Busy(0.0));
                self.render(resolution)?;
                self.respond()
            }
            ProcessorRequest::Export { .. } => todo!(),
            ProcessorRequest::SetLineNumber(line_number) => {
                self.state_change(ProcessingState::Busy(0.0));
                self.context.line_number = line_number;
                self.respond()
            }
        }
    }

    /// We can render if the processor is initialized and we have evaluated some source into a model.
    pub(crate) fn can_render(&self) -> bool {
        self.context.initialized && self.context.model.is_some()
    }

    pub(crate) fn eval(&mut self) -> miette::Result<()> {
        match &self.context.source_file {
            Some(source_file) => {
                // resolve the file
                let resolve_context = microcad_lang::resolve::ResolveContext::create(
                    source_file.clone(),
                    &self.context.search_paths,
                    Some(microcad_builtin::builtin_module()),
                    microcad_lang::diag::DiagHandler::default(),
                )?;

                let mut eval_context = microcad_lang::eval::EvalContext::new(
                    resolve_context,
                    microcad_lang::eval::Stdout::new(),
                    microcad_builtin::builtin_exporters(),
                    microcad_builtin::builtin_importers(),
                );

                match eval_context.eval() {
                    Ok(model) => {
                        self.context.model = model;
                        if eval_context.has_errors() {
                            self.state_change(ProcessingState::Error);
                            return Err(miette::miette!("Eval error"));
                        }
                    }
                    Err(err) => {
                        log::error!("Eval error {err}");
                        self.state_change(ProcessingState::Error);
                        return Err(err.into());
                    }
                }

                Ok(())
            }
            None => {
                self.state_change(ProcessingState::Error);
                Err(miette::miette!("No source code to evaluate."))
            }
        }
    }

    /// Render geometry from model.
    fn render(&mut self, resolution: Option<RenderResolution>) -> miette::Result<()> {
        if self.can_render() {
            let resolution = match resolution {
                Some(resolution) => resolution,
                None => self.context.resolution.clone(),
            };
            let model = self.context.model.as_ref().expect("Model");
            let (tx, rx) = std::sync::mpsc::channel();

            let mut render_context = RenderContext::new(
                model,
                resolution.clone(),
                Some(self.context.render_cache.clone()),
                Some(tx),
            )?;

            let sender = self.response_sender.clone();
            std::thread::spawn(move || {
                while let Ok(progress) = rx.recv() {
                    sender
                        .send(ProcessorResponse::StateChanged(ProcessingState::Busy(
                            progress,
                        )))
                        .expect("No error");
                }
            });

            let _: Model = model.render_with_context(&mut render_context)?;

            // Remove unused cache items.
            {
                log::info!("Render cache");
                let mut cache = self.context.render_cache.borrow_mut();
                cache.garbage_collection();
            }

            self.context.resolution = resolution;
            Ok(())
        } else {
            self.state_change(ProcessingState::Error);

            Err(miette::miette!("Could not render model."))
        }
    }

    /// Update the model instances and generate processor responses.
    fn respond(&mut self) -> miette::Result<Vec<ProcessorResponse>> {
        if let Some(model) = self.context.model.clone() {
            let mut responses = Vec::new();
            responses.push(ProcessorResponse::RemoveModelInstances(
                self.context.instance_registry.fetch_model_uuids(),
            ));

            self.context.instance_registry.clear_model_uuids();
            self.generate_responses(&model, &mut responses);
            log::info!("{} responses", responses.len());

            responses.push(ProcessorResponse::UpdateMaterials(
                self.context.instance_registry.fetch_model_uuids(),
            ));

            responses.push(ProcessorResponse::SpawnModelInstances(
                self.context.instance_registry.fetch_model_uuids(),
            ));

            self.state_change(ProcessingState::Idle);

            Ok(responses)
        } else {
            Err(miette::miette!("No model to draw."))
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
                let uuid = crate::processor::registry::generate_model_geometry_output_uuid(model);
                let output = model_.output();
                let mut recurse = false;

                // Add a new mesh asset, when we do not have geometry with a uuid in the cache.
                if !self
                    .context
                    .instance_registry
                    .contains_geometry_output(&uuid)
                {
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
                            self.context.instance_registry.insert_geometry_output(uuid);
                            responses.push(ProcessorResponse::NewMeshAsset(uuid, mesh));
                        }
                        None => {
                            recurse = true;
                        }
                    }
                }

                let uuid = crate::processor::registry::generate_model_uuid(model);
                if !self.context.instance_registry.contains_model(&uuid) {
                    self.context.instance_registry.insert_model(uuid);

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
