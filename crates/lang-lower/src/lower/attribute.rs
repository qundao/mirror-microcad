// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::lower::extract_statements;
use crate::{Lower, LowerContext, LowerResult, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

/// Helper function to get outer attributes
pub fn outer_with_doc(
    doc: &ast::DocBlock,
    attr: &Vec<ast::Attribute>,
    context: &mut LowerContext,
) -> LowerResult<ir::Attributes> {
    let mut attr = ir::Attributes::lower(attr, context)?;
    attr.doc = ir::DocBlock::lower(doc, context)?;
    Ok(attr)
}

fn extract_attributes<'a, T, F>(
    i: impl Iterator<Item = &'a ast::Attribute>,
    mut f: F,
) -> LowerResult<Box<[T]>>
where
    F: FnMut(&ast::AttributeCommand) -> LowerResult<Option<T>>,
{
    let mut items = Vec::new();
    i.flat_map(|attr| attr.commands.iter())
        .try_for_each(|cmd| -> LowerResult<()> {
            match f(cmd)? {
                Some(item) => Ok(items.push(item)),
                None => Ok(()),
            }
        })?;

    Ok(items.into_boxed_slice())
}

impl Lower<Vec<ast::Attribute>> for ir::Attributes {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        // Generate outer attributes without doc
        Ok(Self {
            doc: ir::DocBlock::default(),
            meta: Box::<[ir::Meta]>::lower(node, context)?,
            commands: Box::<[ir::Command]>::lower(node, context)?,
            tags: Box::<[ir::Tag]>::lower(node, context)?,
            is_inner: false,
        })
    }
}

impl Lower<ast::DocBlock> for ir::DocBlock {
    fn lower(node: &ast::DocBlock, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(Refer::new(
            node.lines.clone().into_boxed_slice(),
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<ast::StatementList> for ir::DocBlock {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        // This does not check if statements are allowed in this context
        Ok(Self(Refer::new(
            extract_statements(node, |stmt| {
                Ok(match stmt {
                    ast::Statement::InnerDocComment(inner_doc_comment) => {
                        Some(inner_doc_comment.line.clone())
                    }
                    _ => None,
                })
            })?,
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<Vec<ast::Attribute>> for Box<[ir::Meta]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        extract_attributes(node.into_iter(), |cmd| -> LowerResult<_> {
            Ok(match cmd {
                ast::AttributeCommand::Assignment(local_assignment) => {
                    Some(ir::Meta::lower(local_assignment, context)?)
                }
                _ => None,
            })
        })
    }
}

impl Lower<ast::LocalAssignment> for ir::Meta {
    fn lower(node: &ast::LocalAssignment, context: &mut LowerContext) -> LowerResult<Self> {
        let identifier = ir::Identifier::lower(&node.name, context)?;
        Ok(ir::Meta {
            name: ir::QualifiedName::new(vec![identifier], context.src_ref(&node.name.span)),
            expr: ir::ConstantExpression::lower(&node.value, context)?,
        })
    }
}

impl Lower<ast::StatementList> for Box<[ir::Meta]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        extract_attributes(
            node.statements.iter().filter_map(|(stmt, _)| match stmt {
                ast::Statement::InnerAttribute(attribute) => Some(attribute),
                _ => None,
            }),
            |cmd| -> LowerResult<_> {
                Ok(match cmd {
                    ast::AttributeCommand::Assignment(local_assignment) => {
                        Some(ir::Meta::lower(local_assignment, context)?)
                    }
                    _ => None,
                })
            },
        )
    }
}

impl Lower<ast::Call> for ir::Command {
    fn lower(node: &ast::Call, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            name: ir::QualifiedName::lower(&node.name, context)?,
            argument_list: ir::ArgumentList::lower(&node.arguments, context)?,
            src_ref: context.src_ref(&node.span),
        })
    }
}

impl Lower<Vec<ast::Attribute>> for Box<[ir::Command]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        extract_attributes(node.iter(), |cmd| -> LowerResult<_> {
            Ok(match cmd {
                ast::AttributeCommand::Call(call) => Some(ir::Command::lower(call, context)?),
                _ => None,
            })
        })
    }
}

impl Lower<ast::StatementList> for Box<[ir::Command]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        extract_attributes(
            node.statements.iter().filter_map(|(stmt, _)| match stmt {
                ast::Statement::InnerAttribute(attribute) => Some(attribute),
                _ => None,
            }),
            |cmd| -> LowerResult<_> {
                Ok(match cmd {
                    ast::AttributeCommand::Call(call) => Some(ir::Command::lower(call, context)?),
                    _ => None,
                })
            },
        )
    }
}

impl Lower<ast::Identifier> for ir::Tag {
    fn lower(node: &ast::Identifier, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            name: ir::Identifier::lower(node, context)?,
        })
    }
}

impl Lower<Vec<ast::Attribute>> for Box<[ir::Tag]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        extract_attributes(node.iter(), |cmd| -> LowerResult<_> {
            Ok(match cmd {
                ast::AttributeCommand::Ident(ident) => Some(ir::Tag::lower(ident, context)?),
                _ => None,
            })
        })
    }
}

impl Lower<ast::StatementList> for Box<[ir::Tag]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        extract_attributes(
            node.statements.iter().filter_map(|(stmt, _)| match stmt {
                ast::Statement::InnerAttribute(attribute) => Some(attribute),
                _ => None,
            }),
            |cmd| -> LowerResult<_> {
                Ok(match cmd {
                    ast::AttributeCommand::Ident(ident) => Some(ir::Tag::lower(ident, context)?),
                    _ => None,
                })
            },
        )
    }
}

impl Lower<ast::WorkbenchDefinition> for ir::Attributes {
    fn lower(node: &ast::WorkbenchDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        outer_with_doc(&node.doc, &node.attributes, context)
    }
}

/// Lower inner attributes
impl Lower<ast::StatementList> for ir::Attributes {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self {
            doc: ir::DocBlock::lower(node, context)?,
            meta: Box::<[ir::Meta]>::lower(node, context)?,
            commands: Box::<[ir::Command]>::lower(node, context)?,
            tags: Box::<[ir::Tag]>::lower(node, context)?,
            is_inner: true,
        })
    }
}
