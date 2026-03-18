// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_lang_base::{SrcRef, SrcReferrer};

use crate::{builtin::*, doc::Doc, syntax::*, value::*};

/// Retrieve symbol information.
pub trait Info {
    /// Get symbol information.
    fn info(&self) -> SymbolInfo;
}

/// Single parameter information in human readable form.
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
            id: param.id().to_string(),
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
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl SymbolInfo {
    pub(super) fn new_local(id: &Identifier) -> Self {
        SymbolInfo {
            id: id.to_string(),
            kind: "local value".into(),
            ..Default::default()
        }
    }
}

impl From<&Rc<Assignment>> for SymbolInfo {
    fn from(def: &Rc<Assignment>) -> Self {
        SymbolInfo {
            id: def.id().to_string(),
            kind: "Assignment".into(),
            doc: Some(def.doc()),
            signatures: vec![],
            src_ref: def.src_ref(),
        }
    }
}

impl From<&Rc<SourceFile>> for SymbolInfo {
    fn from(def: &Rc<SourceFile>) -> Self {
        SymbolInfo {
            id: def.id().to_string(),
            kind: "SourceFile".into(),
            doc: Some(def.doc()),
            signatures: vec![],
            src_ref: def.src_ref(),
        }
    }
}

impl From<&Rc<ModuleDefinition>> for SymbolInfo {
    fn from(def: &Rc<ModuleDefinition>) -> Self {
        SymbolInfo {
            id: def.id().to_string(),
            kind: "ModuleDefinition".into(),
            doc: Some(def.doc()),
            signatures: vec![],
            src_ref: def.src_ref(),
        }
    }
}

impl From<&Rc<WorkbenchDefinition>> for SymbolInfo {
    fn from(def: &Rc<WorkbenchDefinition>) -> Self {
        SymbolInfo {
            id: def.id().to_string(),
            kind: def.kind.to_string(),
            doc: Some(def.doc()),
            signatures: def
                .inits()
                .map(|init| SignatureInfo {
                    params: init.parameters.iter().map(|p| p.into()).collect(),
                    doc: Some(init.doc()),
                })
                .collect(),
            src_ref: def.src_ref(),
        }
    }
}

impl From<&Rc<FunctionDefinition>> for SymbolInfo {
    fn from(def: &Rc<FunctionDefinition>) -> Self {
        SymbolInfo {
            id: def.id().to_string(),
            kind: "Function".into(),
            doc: Some(def.doc()),
            signatures: vec![SignatureInfo {
                params: def.signature.parameters.iter().map(|p| p.into()).collect(),
                doc: Some(def.doc()),
            }],
            src_ref: def.src_ref(),
        }
    }
}

impl From<&Builtin> for SymbolInfo {
    fn from(def: &Builtin) -> Self {
        use crate::doc::Doc;
        SymbolInfo {
            id: def.id().to_string(),
            kind: "Builtin".into(),
            doc: Some(def.doc()),
            signatures: vec![],
            src_ref: SrcRef(None),
        }
    }
}
