// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{builtin::Builtin, rc::*, syntax::*, value::*};

/// Retrieve symbol information.
pub trait Info {
    /// Get symbol information.
    fn info(&self) -> SymbolInfo;
}

/// Single parameter information ion human readable form.
pub struct ParameterInfo {
    /// Parameter ID.
    pub id: String,
    /// Type if given.
    pub ty: Option<String>,
    /// Default value if given.
    pub def: Option<String>,
}

impl From<&Parameter> for ParameterInfo {
    fn from(param: &Parameter) -> Self {
        Self {
            id: param.id.to_string(),
            ty: param.specified_type.clone().map(|ty| ty.to_string()),
            def: param.default_value.clone().map(|def| def.to_string()),
        }
    }
}

impl From<(&Identifier, &ParameterValue)> for ParameterInfo {
    fn from(param: (&Identifier, &ParameterValue)) -> Self {
        Self {
            id: param.0.to_string(),
            ty: param.1.specified_type.clone().map(|ty| ty.to_string()),
            def: param.1.default_value.clone().map(|def| def.to_string()),
        }
    }
}

/// Call signature information in human readable form.
pub struct SignatureInfo {
    /// Parameters of the call.
    pub params: Vec<ParameterInfo>,
    /// Documentation.
    pub doc: Option<DocBlock>,
}

/// Symbol information in human readable form.
#[derive(Default)]
pub struct SymbolInfo {
    /// ID of the symbol.
    pub id: String,
    /// Human readable symbol kind.
    pub kind: String,
    /// Optional documentation block.
    pub doc: Option<DocBlock>,
    /// Parameters and alternative parameters if any.
    pub signatures: Vec<SignatureInfo>,
}

impl SymbolInfo {
    pub(super) fn new_constant(_: &Visibility, id: &Identifier, _: &Value) -> Self {
        SymbolInfo {
            id: id.to_string(),
            kind: "Constant".into(),
            ..Default::default()
        }
    }

    pub(super) fn new_arg(_d: &Identifier, _: &Value) -> Self {
        SymbolInfo {
            kind: "Argument".into(),
            ..Default::default()
        }
    }
}

impl From<&Rc<Assignment>> for SymbolInfo {
    fn from(def: &Rc<Assignment>) -> Self {
        SymbolInfo {
            id: def.id.to_string(),
            kind: "Assignment".into(),
            doc: def.doc(),
            signatures: vec![],
        }
    }
}

impl From<&Rc<SourceFile>> for SymbolInfo {
    fn from(def: &Rc<SourceFile>) -> Self {
        SymbolInfo {
            id: def.id().to_string(),
            kind: "SourceFile".into(),
            doc: def.doc(),
            signatures: vec![],
        }
    }
}

impl From<&Rc<ModuleDefinition>> for SymbolInfo {
    fn from(def: &Rc<ModuleDefinition>) -> Self {
        SymbolInfo {
            id: def.id.to_string(),
            kind: "SourceFile".into(),
            doc: def.doc(),
            signatures: vec![],
        }
    }
}

impl From<&Rc<WorkbenchDefinition>> for SymbolInfo {
    fn from(def: &Rc<WorkbenchDefinition>) -> Self {
        SymbolInfo {
            id: def.id.to_string(),
            kind: def.kind.to_string(),
            doc: def.doc(),
            signatures: def
                .inits()
                .map(|init| SignatureInfo {
                    params: init.parameters.iter().map(|p| p.into()).collect(),
                    doc: init.doc(),
                })
                .collect(),
        }
    }
}

impl From<&Rc<FunctionDefinition>> for SymbolInfo {
    fn from(def: &Rc<FunctionDefinition>) -> Self {
        SymbolInfo {
            id: def.id.to_string(),
            kind: "Function".into(),
            doc: def.doc(),
            signatures: vec![SignatureInfo {
                params: def.signature.parameters.iter().map(|p| p.into()).collect(),
                doc: def.doc(),
            }],
        }
    }
}

impl From<&Rc<Builtin>> for SymbolInfo {
    fn from(def: &Rc<Builtin>) -> Self {
        SymbolInfo {
            id: def.id.to_string(),
            kind: "Builtin".into(),
            doc: def.doc.clone(),
            signatures: vec![SignatureInfo {
                params: def.parameters.iter().map(|p| p.into()).collect(),
                doc: def.doc.clone(),
            }],
        }
    }
}
