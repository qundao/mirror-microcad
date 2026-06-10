// Copyright © 2025-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Lower, LowerContext, LowerResult, ir};

use microcad_lang_base::Refer;
use microcad_lang_parse::ast;

/// Extracts and maps specific variants out of a statement collection tuple list.
///
/// Does not check if the statements are actually valid in this context.
pub fn extract_statements<F, T>(statements: &ast::StatementList, mut extractor: F) -> Vec<T>
where
    F: FnMut(&ast::Statement) -> Option<T>,
{
    statements
        .statements
        .iter()
        .filter_map(|(statement, _)| extractor(statement))
        .collect()
}

/// Helper function to get outer attributes
pub fn outer_attr(
    doc: &ast::DocBlock,
    attr: &Vec<ast::Attribute>,
    context: &mut LowerContext,
) -> LowerResult<ir::Attributes> {
    Ok(ir::Attributes {
        doc: ir::DocBlock::lower(doc, context)?,
        meta: Box::<[ir::Meta]>::lower(attr, context)?,
        commands: Box::<[ir::Command]>::lower(attr, context)?,
        tags: Box::<[ir::Tag]>::lower(attr, context)?,
        is_inner: false,
    })
}

impl Lower<ast::DocBlock> for ir::DocBlock {
    fn lower(node: &ast::DocBlock, context: &mut LowerContext) -> LowerResult<Self> {
        Ok(Self(Refer::new(
            node.lines.clone(),
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<ast::StatementList> for ir::DocBlock {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        // This does not check if statements are allowed in this context
        Ok(Self(Refer::new(
            extract_statements(node, |stmt| match stmt {
                ast::Statement::InnerDocComment(inner_doc_comment) => {
                    Some(inner_doc_comment.line.clone())
                }
                _ => None,
            }),
            context.src_ref(&node.span),
        )))
    }
}

impl Lower<&[ast::Attribute]> for Box<[ir::Meta]> {
    fn lower(node: &[&ast::Attribute], context: &mut LowerContext) -> LowerResult<Self> {
        let mut meta = Vec::new();

        node.iter()
            .flat_map(|attr| attr.commands.iter())
            .try_for_each(|cmd| {
                match cmd {
                    // 1. Process Metadata Assignments: #[color = "red"]
                    ast::AttributeCommand::Assignment(local_assignment) => {
                        let identifier = ir::Identifier::lower(local_assignment.name, context)?;
                        meta.push(ir::Meta {
                            name: ir::QualifiedName::new(
                                vec![identifier],
                                context.src_ref(&local_assignment.name.span),
                            ),
                            expr: ir::ConstantExpression::lower(&local_assignment.value, context)?,
                        });
                        Ok(())
                    }
                    _ => Ok(()),
                }
            })?;

        Ok(meta.into_boxed_slice())
    }
}

impl Lower<Vec<ast::Attribute>> for Box<[ir::Meta]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        Self::lower(&node.as_slice(), context)
    }
}

impl Lower<ast::StatementList> for Box<[ir::Meta]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        let attr: Vec<ast::Attribute> = extract_statements(node, |stmt| match stmt {
            ast::Statement::InnerAttribute(attribute) => Some(*attribute),
            _ => None,
        });

        Self::lower(&attr, context)
    }
}

impl Lower<&[ast::Attribute]> for Box<[ir::Command]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        let mut commands = Vec::new();

        node.iter()
            .flat_map(|attr| attr.commands.iter())
            .try_for_each(|cmd| {
                match cmd {
                    // 1. Process Metadata Assignments: #[color = "red"]
                    ast::AttributeCommand::Call(call) => {
                        commands.push(ir::Command {
                            name: ir::QualifiedName::lower(&call.name, context)?,
                            argument_list: ir::ArgumentList::lower(&call.arguments, context)?,
                            src_ref: context.src_ref(&call.span),
                        });
                        Ok(())
                    }
                    _ => Ok(()),
                }
            })?;

        Ok(commands.into_boxed_slice())
    }
}

impl Lower<Vec<ast::Attribute>> for Box<[ir::Command]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        Self::lower(&node.as_slice(), context)
    }
}

impl Lower<ast::StatementList> for Box<[ir::Command]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        let attr: Vec<ast::Attribute> = extract_statements(node, |stmt| match stmt {
            ast::Statement::InnerAttribute(attribute) => Some(*attribute),
            _ => None,
        });

        Self::lower(&attr, context)
    }
}

impl Lower<&[ast::Attribute]> for Box<[ir::Tag]> {
    fn lower(node: &[ast::Attribute], context: &mut LowerContext) -> LowerResult<Self> {
        let mut commands = Vec::new();

        node.iter()
            .flat_map(|attr| attr.commands.iter())
            .try_for_each(|cmd| {
                match cmd {
                    // 1. Process Metadata Assignments: #[color = "red"]
                    ast::AttributeCommand::Ident(tag) => {
                        commands.push(ir::Tag {
                            name: ir::Identifier::lower(&tag.name, context)?,
                        });
                        Ok(())
                    }
                    _ => Ok(()),
                }
            })?;

        Ok(commands.into_boxed_slice())
    }
}
impl Lower<Vec<ast::Attribute>> for Box<[ir::Tag]> {
    fn lower(node: &Vec<ast::Attribute>, context: &mut LowerContext) -> LowerResult<Self> {
        Self::lower(&node.as_slice(), context)
    }
}

impl Lower<ast::StatementList> for Box<[ir::Tag]> {
    fn lower(node: &ast::StatementList, context: &mut LowerContext) -> LowerResult<Self> {
        let attr: Vec<ast::Attribute> = extract_statements(node, |stmt| match stmt {
            ast::Statement::InnerAttribute(attribute) => Some(*attribute),
            _ => None,
        });

        Self::lower(&attr, context)
    }
}

impl Lower<ast::FunctionDefinition> for ir::Attributes {
    fn lower(node: &ast::FunctionDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        outer_attr(&node.doc, &node.attributes, context)
    }
}

impl Lower<ast::WorkbenchDefinition> for ir::Attributes {
    fn lower(node: &ast::WorkbenchDefinition, context: &mut LowerContext) -> LowerResult<Self> {
        outer_attr(&node.doc, &node.attributes, context)
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
