// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*};

impl Parse for AttributeCommand {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_command);

        for inner in pair.inner() {
            match inner.as_rule() {
                Rule::attribute_assigment => {
                    return Ok(Self::Assigment {
                        name: crate::find_rule!(inner, identifier)?,
                        value: crate::find_rule!(inner, expression)?,
                        src_ref: pair.src_ref(),
                    });
                }
                Rule::attribute_ident => {
                    return Ok(Self::Ident(crate::find_rule!(inner, identifier)?));
                }
                Rule::attribute_call => {
                    let call: Call = crate::find_rule!(inner, call)?;
                    if call.name.as_identifier().is_none() {
                        return Err(ParseError::InvalidAttributeCall(call.name));
                    }
                    return Ok(Self::Call(call));
                }
                _ => {}
            }
        }

        unreachable!()
    }
}

impl Parse for Attribute {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rules(&pair, &[Rule::attribute, Rule::inner_attribute]);

        let mut commands = Vec::new();
        for pair in pair.inner() {
            if pair.as_rule() == Rule::attribute_command {
                commands.push(AttributeCommand::parse(pair)?);
            }
        }

        Ok(Self {
            commands,
            is_inner: pair.as_rule() == Rule::inner_attribute,
            src_ref: pair.src_ref(),
        })
    }
}

impl Parse for AttributeList {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::attribute_list);
        let mut attribute_list = AttributeList::default();

        for pair in pair.inner() {
            match pair.as_rule() {
                Rule::attribute => {
                    attribute_list.push(Attribute::parse(pair)?);
                }
                Rule::COMMENT | Rule::doc_comment => {}
                rule => unreachable!("Unexpected element {rule:?}"),
            }
        }

        Ok(attribute_list)
    }
}
