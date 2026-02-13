// Copyright © 2025-2026 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Convert µcad types into Slint view model types.

use crate::*;

use microcad_lang::model::{Creator, Element, Model};
use slint::ToSharedString;

pub trait ItemsFromTree<T, D = usize>: Sized
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
pub fn model_rc_from_items<T: Sized + Clone + 'static>(items: Vec<T>) -> slint::ModelRc<T> {
    slint::ModelRc::new(VecModel::from(items))
}

impl From<&SrcRef> for VM_SrcRef {
    fn from(src_ref: &SrcRef) -> Self {
        let src_ref = src_ref.as_ref();
        Self {
            line: src_ref.map(|src_ref| src_ref.at.line).unwrap_or_default() as i32,
            col: src_ref.map(|src_ref| src_ref.at.col).unwrap_or_default() as i32,
            source_hash: src_ref
                .map(|src_ref| hash_to_shared_string(src_ref.source_file_hash))
                .unwrap_or_default(),
        }
    }
}

impl From<&Refer<Element>> for VM_Element {
    fn from(value: &Refer<Element>) -> Self {
        Self {
            name: value.value.to_string().into(),
            src_ref: (&value.src_ref).into(),
        }
    }
}

impl From<&DocBlock> for VM_DocBlock {
    fn from(doc: &DocBlock) -> Self {
        Self {
            lines: doc.0.join("\n").to_shared_string(),
            src_ref: (&doc.src_ref()).into(),
        }
    }
}

impl From<SymbolInfo> for VM_SymbolInfo {
    fn from(info: SymbolInfo) -> Self {
        Self {
            id: info.id.into(),
            kind: info.kind.into(),
            doc: info.doc.as_ref().map(|doc| doc.into()).unwrap_or_default(),
            src_ref: (&info.src_ref).into(),
        }
    }
}

impl From<Option<&Creator>> for VM_Creator {
    fn from(creator: Option<&Creator>) -> Self {
        use microcad_lang::resolve::Info;
        match creator {
            Some(creator) => Self {
                symbol: creator.symbol.info().into(),
                src_ref: (&creator.symbol.src_ref()).into(),
            },
            None => Self::default(),
        }
    }
}

impl ItemsFromTree<Model> for ModelTreeModelItem {
    fn _from_tree(model: &Model, items: &mut Vec<Self>, depth: usize) {
        let model_ = model.borrow();

        items.push(Self {
            depth: depth as i32,
            element: (&model_.element).into(),
            creator: model_.element.creator().into(),
        });
        model_
            .children()
            .for_each(|model| Self::_from_tree(model, items, depth + 1))
    }
}

impl ItemsFromTree<Symbol> for SymbolTreeModelItem {
    fn _from_tree(symbol: &Symbol, items: &mut Vec<Self>, depth: usize) {
        use microcad_lang::src_ref::SrcReferrer;

        items.push(Self {
            depth: depth as i32,
            name: symbol.full_name().to_string().into(),
            source_hash: symbol.src_ref().source_hash() as i32,
        });

        symbol.with_children(|(_, symbol)| {
            Self::_from_tree(symbol, items, depth + 1);
        })
    }
}

/// Split a source code string into a list of source code model items.
pub fn split_source_code(source: &str) -> Vec<SourceCodeModelItem> {
    let mut items = Vec::new();
    let mut byte_index = 0;

    for (line_number, line) in source.split_inclusive('\n').enumerate() {
        // `split_inclusive('\n')` keeps the newline at the end of each line,
        // which helps preserve correct byte ranges and offsets.
        let line_bytes = line.as_bytes();
        let line_len = line_bytes.len();

        items.push(SourceCodeModelItem {
            line: line.to_string().into(),
            line_number: line_number as i32,
            byte_range_start: byte_index as i32,
            byte_range_end: (byte_index + line_len) as i32,
            ..Default::default()
        });

        byte_index += line_len;
    }

    // Handle case where the last line does not end with a newline
    if !source.ends_with('\n') && !source.is_empty() {
        if let Some(last_line) = source.lines().last() {
            let line_len = last_line.len();
            let line_start = source.len() - line_len;

            items.push(SourceCodeModelItem {
                line: last_line.to_string().into(),
                line_number: items.len() as i32,
                byte_range_start: line_start as i32,
                byte_range_end: source.len() as i32,
                ..Default::default()
            });
        }
    }

    items
}

pub fn hash_to_shared_string(hash: u64) -> slint::SharedString {
    format!("{hash:016X}").into()
}
