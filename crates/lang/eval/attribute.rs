// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{
    Id,
    builtin::ExporterAccess,
    eval::{self, *},
    model::{Attributes, CustomCommand, ExportCommand, MeasureCommand, ResolutionAttribute},
    parameter,
    syntax::{self, *},
};
use miette::Diagnostic;
use std::str::FromStr;

use microcad_core::{Color, Length, RenderResolution, Size2};
use thiserror::Error;

/// Error type for attributes.
#[derive(Debug, Error, Diagnostic)]
pub enum AttributeError {
    /// Unknown attribute.
    #[error("Attribute not supported: {0}")]
    NotSupported(Identifier),

    /// Attribute cannot be assigned to an expression.
    #[error("Cannot assign attribute to expression `{0}`")]
    CannotAssignAttribute(String),

    /// The attribute was not found.
    #[error("Not found: {0}")]
    NotFound(Identifier),

    /// Invalid command.
    #[error("Invalid command list for attribute `{0}`")]
    InvalidCommand(Identifier),
}

impl Eval<Option<ExportCommand>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<ExportCommand>> {
        match self {
            AttributeCommand::Call(call) => {
                match ArgumentMatch::find_match(
                    &call.argument_list.eval(context)?,
                    &[
                        parameter!(filename: String),
                        parameter!(resolution: Length = Length::mm(0.1)),
                        (
                            Identifier::no_ref("size"),
                            eval::ParameterValue {
                                specified_type: Some(Type::Tuple(Box::new(TupleType::new_size2()))),
                                default_value: Some(Value::Tuple(Box::new(Size2::A4.into()))),
                                src_ref: SrcRef(None),
                            },
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ) {
                    Ok(arguments) => {
                        let filename: std::path::PathBuf =
                            arguments.get::<String>("filename").into();
                        let id: Option<Id> = if let Ok(id) = arguments.by_str::<String>("id") {
                            Some(id.into())
                        } else {
                            None
                        };
                        let resolution = RenderResolution::new(
                            arguments.get::<&Value>("resolution").try_scalar()?,
                        );

                        match context.find_exporter(&filename, &id) {
                            Ok(exporter) => Ok(Some(ExportCommand {
                                filename,
                                exporter,
                                resolution,
                            })),
                            Err(err) => {
                                context.warning(self, err)?;
                                Ok(None)
                            }
                        }
                    }
                    Err(err) => {
                        context.warning(self, err)?;
                        Ok(None)
                    }
                }
            }
            AttributeCommand::Assigment { value, .. } => {
                let value: Value = value.eval(context)?;
                match value {
                    Value::String(filename) => {
                        let filename = std::path::PathBuf::from(filename);
                        match context.find_exporter(&filename, &None) {
                            Ok(exporter) => Ok(Some(ExportCommand {
                                filename,
                                resolution: RenderResolution::default(),
                                exporter,
                            })),
                            Err(err) => {
                                context.warning(self, err)?;
                                Ok(None)
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
            }
            _ => Ok(None),
        }
    }
}

impl Eval<Vec<ExportCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Vec<ExportCommand>> {
        self.commands
            .iter()
            .try_fold(Vec::new(), |mut commands, attribute| {
                assert_eq!(attribute.name().id().as_str(), "export");
                if let Some(export_command) = attribute.eval(context)? {
                    commands.push(export_command)
                }
                Ok(commands)
            })
    }
}

impl Eval<Vec<MeasureCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Vec<MeasureCommand>> {
        let mut commands = Vec::new();

        for command in &self.commands {
            match command {
                AttributeCommand::Call(_) => {
                    match command.name().id().as_str() {
                        "width" => commands.push(MeasureCommand::Width),
                        "height" => commands.push(MeasureCommand::Height),
                        "size" => commands.push(MeasureCommand::Size),
                        _ => context
                            .warning(self, AttributeError::InvalidCommand(command.name().clone()))?,
                    }
                }
                _ => unimplemented!(),
            }
        }

        Ok(commands)
    }
}

impl Eval<Vec<CustomCommand>> for syntax::Attribute {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Vec<CustomCommand>> {
        let mut commands = Vec::new();
        for command in &self.commands {
            match command {
                AttributeCommand::Call(call) => {
                    match context.exporters().exporter_by_id(command.name().id()) {
                        Ok(exporter) => {
                            match ArgumentMatch::find_match(
                                &call.argument_list.eval(context)?,
                                &exporter.model_parameters(),
                            ) {
                                Ok(tuple) => commands.push(CustomCommand {
                                    id: command.name().clone(),
                                    arguments: Box::new(tuple),
                                }),
                                Err(err) => {
                                    context.warning(self, err)?;
                                }
                            }
                        }
                        Err(err) => {
                            context.warning(self, err)?;
                        }
                    }
                }
                AttributeCommand::Assigment { name, .. } => {
                    match context.exporters().exporter_by_id(name.id()) {
                        Ok(_) => commands.push(CustomCommand {
                            id: command.name().clone(),
                            arguments: Box::new(Tuple::default()),
                        }),
                        Err(err) => {
                            context.warning(self, err)?;
                        }
                    }
                }
                _ => unimplemented!(),
            }
        }

        Ok(commands)
    }
}

impl Eval<Option<Color>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<Color>> {
        match self {
            // Get color from a tuple or string.
            AttributeCommand::Assigment { value, .. } => {
                let value: Value = value.eval(context)?;
                match value {
                    // Color from string: color = "red"
                    Value::String(s) => match Color::from_str(&s) {
                        Ok(color) => Ok(Some(color)),
                        Err(err) => {
                            context.warning(self, err)?;
                            Ok(None)
                        }
                    },
                    // Color from tuple: color = (r = 1.0, g = 1.0, b = 1.0, a = 1.0)
                    Value::Tuple(tuple) => match Color::try_from(tuple.as_ref()) {
                        Ok(color) => Ok(Some(color)),
                        Err(err) => {
                            context.warning(self, err)?;
                            Ok(None)
                        }
                    },
                    _ => {
                        context.warning(
                            self,
                            AttributeError::InvalidCommand(Identifier::no_ref("color")),
                        )?;
                        Ok(None)
                    }
                }
            }
            _ => todo!(),
        }
    }
}

impl Eval<Option<ResolutionAttribute>> for syntax::AttributeCommand {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<ResolutionAttribute>> {
        match self {
            AttributeCommand::Assigment { value, .. } => {
                let value: Value = value.eval(context)?;
                match value {
                    Value::Quantity(qty) => match qty.quantity_type {
                        QuantityType::Scalar => Ok(Some(ResolutionAttribute::Relative(qty.value))),
                        QuantityType::Length => Ok(Some(ResolutionAttribute::Absolute(qty.value))),
                        _ => unimplemented!(),
                    },
                    _ => todo!("Error handling"),
                }
            }
            _ => {
                context.warning(
                    self,
                    AttributeError::InvalidCommand(Identifier::no_ref("resolution")),
                )?;
                Ok(None)
            }
        }
    }
}

impl Eval<Option<Size2>> for syntax::AttributeCommand {
    fn eval(&self, _: &mut EvalContext) -> EvalResult<Option<Size2>> {
        todo!("Get Size2, e.g. `size = (width = 10mm, height = 10mm) from AttributeCommand")
    }
}

macro_rules! eval_to_attribute {
    ($id:ident: $ty:ty) => {
        impl Eval<Option<$ty>> for syntax::Attribute {
            fn eval(&self, context: &mut EvalContext) -> EvalResult<Option<$ty>> {
                let command = self.commands.first().expect("empty attribute");
                assert_eq!(
                    command.name().id().as_str(),
                    stringify!($id))
                ;
                match self.single_command() {
                    Some(command) => Ok(command.eval(context)?),
                    None => {
                        context.warning(
                            self,
                            AttributeError::InvalidCommand(command.name().clone()),
                        )?;
                        Ok(None)
                    }
                }
            }
        }
    };
}

eval_to_attribute!(color: Color);
eval_to_attribute!(resolution: ResolutionAttribute);
eval_to_attribute!(size: Size2);

impl Eval<Vec<crate::model::Attribute>> for syntax::Attribute {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Vec<crate::model::Attribute>> {
        use crate::model::Attribute as Attr;
        self.commands
            .iter()
            .map(|command| {
                let id = command.name().id().as_str();
                Ok(match id {
                    "color" => match self.eval(context)? {
                        Some(color) => vec![Attr::Color(color)],
                        None => Default::default(),
                    },
                    "resolution" => match self.eval(context)? {
                        Some(resolution) => vec![Attr::Resolution(resolution)],
                        None => Default::default(),
                    },
                    "size" => match self.eval(context)? {
                        Some(size) => vec![Attr::Size(size)],
                        None => Default::default(),
                    },
                    "export" => {
                        let exports: Vec<ExportCommand> = self.eval(context)?;
                        exports.iter().cloned().map(Attr::Export).collect()
                    }
                    "measure" => {
                        let measures: Vec<MeasureCommand> = self.eval(context)?;
                        measures.iter().cloned().map(Attr::Measure).collect()
                    }
                    _ => {
                        let commands: Vec<CustomCommand> = self.eval(context)?;
                        commands.iter().cloned().map(Attr::Custom).collect()
                    }
                })
            })
            .flat_map(|res| match res {
                Ok(res) => res.into_iter().map(Ok).collect(),
                Err(err) => vec![Err(err)],
            })
            .collect()
    }
}

impl Eval<crate::model::Attributes> for AttributeList {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<crate::model::Attributes> {
        Ok(Attributes(self.iter().try_fold(
            Vec::new(),
            |mut attributes, attribute| -> EvalResult<_> {
                attributes.append(&mut attribute.eval(context)?);
                Ok(attributes)
            },
        )?))
    }
}
