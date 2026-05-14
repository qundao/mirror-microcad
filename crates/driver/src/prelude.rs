// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad driver prelude
//!
//! Preferably include with: `use microcad_driver::prelude as mu;`
//! To use the traits: `use mu::traits::*;`

pub mod core {
    pub use microcad_core::*;
}

pub use core::{Color, Scalar};

pub mod base {
    pub use microcad_lang_base::*;
}

pub use base::{HashId, HashSet, Hashed, RcMut, Refer, SrcRef, Url};

pub mod builtin {
    pub use microcad_builtin::*;
}

pub mod parse {
    pub use microcad_lang_parse::*;
}

pub use parse::{ParseContext, ast};

pub mod lower {
    pub use microcad_lang::lower::*;
}

pub use lower::{LowerContext, ir};

pub mod export {
    pub use microcad_export::*;
}

pub use microcad_lang::model::{Creator, Element, Model, OutputType};
pub use microcad_lang::render::{
    GeometryOutput, ProgressTx, RenderCache, RenderContext, RenderResolution, RenderWithContext,
};
pub use microcad_lang::symbol::{Symbol, SymbolInfo};

pub use crate::config::Config;
pub use crate::document::Document;
pub use crate::session::Session;
pub use crate::watcher::Watcher;

pub use crate::{Report, Result, report};

pub use crate::commands;
pub use crate::document;

pub use crate::install_std;
pub use crate::locate;

pub use crate::commands::{
    CompileParameters, DocGen, DocGenParameters, Export, ExportCommand, ExportParameters, Format,
    FormatParameters, PrintDiagnosticsParameters, Sync, compile::Parse, compile::RenderParameters,
    compile::ResolveParameters,
};

pub mod traits {
    pub use super::base::{ComputedHash, ResourceLocation, SrcReferrer};
    pub use super::commands::{Compile, DocGen, Export, Format, PrintDiagnostics, Sync, compile};
    pub use super::core::{CalcBounds2D, CalcBounds3D};
    pub use microcad_lang::symbol::Info;

    pub use super::lower::Lower;
    pub use super::parse::Parse;
}
