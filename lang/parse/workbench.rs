// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for Refer<WorkbenchKind> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.as_str() {
            "part" => Ok(Refer::new(WorkbenchKind::Part, pair.into())),
            "sketch" => Ok(Refer::new(WorkbenchKind::Sketch, pair.into())),
            "op" => Ok(Refer::new(WorkbenchKind::Operation, pair.into())),
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

impl Parse for Rc<WorkbenchDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(WorkbenchDefinition {
            doc: crate::find_rule_opt!(pair, doc_block),
            visibility: crate::find_rule!(pair, visibility)?,
            attribute_list: crate::find_rule!(pair, attribute_list)?,
            kind: crate::find_rule_exact!(pair, workbench_kind)?,
            id: crate::find_rule!(pair, identifier)?,
            plan: crate::find_rule!(pair, parameter_list)?,
            body: crate::find_rule!(pair, body)?,
        }
        .into())
    }
}
