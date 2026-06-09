// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crossbeam::channel::{Receiver, Sender};
use microcad_driver::prelude as mu;

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

    pub(crate) fn compile(
        &mut self,
        mut document: mu::document::SourceFile,
    ) -> miette::Result<Vec<ProcessorResponse>> {
        use microcad_driver::commands::{Compile, *};

        self.state_change(ProcessingState::Busy(0.0));

        let (tx, rx) = std::sync::mpsc::channel();
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

        let compiler_params = CompileParameters {
            resolve: compile::ResolveParameters {
                search_paths: self.context.search_paths.clone(),
            },
        };
        let render_params = RenderParameters {
            resolution: self.context.resolution.clone(),
            cache: Some(self.context.render_cache.clone()),
            progress_tx: Some(tx),
        };

        let responses = match document
            .compile(compiler_params)
            .and(document.render(render_params))
        {
            Ok(model) => self.respond(model),
            Err(err) => {
                eprintln!(
                    "{}",
                    document.diagnostics_string(&mu::base::DiagRenderOptions {
                        color: true,
                        ..Default::default()
                    }),
                );

                log::error!("{err}");
                self.state_change(ProcessingState::Error);
                Ok(vec![])
            }
        }?;

        self.context.document = Some(document);

        // Remove unused cache items.
        {
            log::info!("Render cache");
            let mut cache = self.context.render_cache.borrow_mut();
            cache.garbage_collection();
        }

        Ok(responses)
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
            ProcessorRequest::CompileFile(path) => {
                self.compile(mu::document::SourceFile::from_file(path)?)
            }
            ProcessorRequest::CompileSource {
                path,
                name: _name,
                source,
            } => self.compile(mu::document::SourceFile::from_source(mu::base::Source {
                url: mu::locate::to_url(path.as_ref().unwrap().to_str().unwrap())?,
                line_offset: 0,
                code: mu::Hashed::new(source),
            })),
            ProcessorRequest::Export { .. } => todo!(),
            ProcessorRequest::SetLineNumber(line_number) => {
                self.state_change(ProcessingState::Busy(0.0));
                self.context.line_number = line_number;
                Ok(vec![])
                //self.respond()
            }
            _ => unreachable!(),
        }
    }

    /// Update the model instances and generate processor responses.
    fn respond(&mut self, model: mu::Model) -> miette::Result<Vec<ProcessorResponse>> {
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
    }

    /// Generate mesh geometry output for model.
    fn generate_responses(&mut self, model: &mu::Model, responses: &mut Vec<ProcessorResponse>) {
        use mu::Element::*;
        match model.render_output_type() {
            mu::OutputType::Geometry2D | mu::OutputType::Geometry3D => {}
            mu::OutputType::NotDetermined | mu::OutputType::InvalidMixed => return,
        }

        let model_ = model.borrow();
        // We only consider output geometries of workpieces and ignore the rest.
        let recurse = match model_.element() {
            Value(_) => true, // Values might produce geometries at some point (to draw Vec2, Vec3 etc. in the scene)
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
                        Some(mu::GeometryOutput::Geometry2D(geometry)) => {
                            Some(geometry.inner.to_bevy_mesh_default())
                        }
                        Some(mu::GeometryOutput::Geometry3D(geometry)) => {
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
