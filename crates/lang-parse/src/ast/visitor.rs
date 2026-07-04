// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang_base::Spanned;

use crate::{Ast, ast};

/// Creates the default visit function implementation for a particular type
macro_rules! define_visit {
    ($fn_name:ident, $type_name:path) => {
        #[doc = concat!("Visits a `", stringify!($type_name), "` with this visitor")]
        fn $fn_name(&mut self, node: &'ast $type_name) -> std::ops::ControlFlow<Self::BreakTy> {
            node.visit(self)
        }
    };
}

pub trait Visit {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        std::ops::ControlFlow::Continue(())
    }
}

impl<T: Visit> Visit for Vec<T> {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        for item in self {
            item.visit(visitor)?;
        }
        std::ops::ControlFlow::Continue(())
    }
}

impl<T: Visit> Visit for Option<T> {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        if let Some(item) = &self {
            item.visit(visitor)?;
        }
        std::ops::ControlFlow::Continue(())
    }
}

impl<T: Visit> Visit for Box<T> {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        self.as_ref().visit(visitor)
    }
}

impl<T: Visit> Visit for Spanned<T> {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        self.value.visit(visitor)
    }
}

// Primitives that don't need visiting

impl Visit for ast::Span {}
impl Visit for ast::DocBlock {}
impl Visit for ast::def::WorkbenchKind {}
impl Visit for ast::def::Visibility {}
impl Visit for ast::Identifier {}
impl Visit for ast::Type {}

impl Visit for ast::Comment {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        visitor.visit_comment(self)
    }
}
impl Visit for ast::Unit {}
impl Visit for ast::BinaryOperator {}
impl Visit for ast::UnaryOperator {}
impl Visit for ast::BoolLiteral {}
impl Visit for ast::FloatLiteral {}
impl Visit for ast::IntegerLiteral {}
impl Visit for ast::StringLiteral {}

impl Visit for ast::Attributes {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        self.0.visit(visitor)
    }
}

impl Visit for ast::StatementList {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        self.extras.visit(visitor)?;
        for (statement, trailing) in &self.statements {
            statement.visit(visitor)?;
            trailing.visit(visitor)?;
        }
        self.tail.visit(visitor)
    }
}

impl Visit for ast::LeadingExtras {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        self.0.visit(visitor)
    }
}

impl Visit for ast::TrailingExtras {
    fn visit<'ast, V>(&'ast self, visitor: &mut V) -> std::ops::ControlFlow<V::BreakTy>
    where
        V: Visitor<'ast> + ?Sized,
    {
        self.0.visit(visitor)
    }
}

pub trait Visitor<'ast> {
    /// Type which will be propagated from the visitor if completing early.
    type BreakTy;

    define_visit!(visit_ast, Ast);
    define_visit!(visit_def_workbench, ast::def::Workbench);
    define_visit!(visit_statement_list, ast::StatementList);
    define_visit!(visit_doc_block, ast::DocBlock);
    define_visit!(visit_def_inline_module, ast::def::InlineModule);
    define_visit!(visit_extras, ast::ItemExtras);
    define_visit!(visit_extra, ast::ItemExtra);
    define_visit!(visit_leading_extras, ast::LeadingExtras);
    define_visit!(visit_trailing_extras, ast::TrailingExtras);
    define_visit!(visit_attributes, ast::Attributes);
    define_visit!(visit_comment, ast::Comment);
    define_visit!(visit_identifier, ast::Identifier);
}

/// This visitor collects all comments (*not* including doc comments).
#[derive(Debug, derive_more::Deref, Default)]
pub struct CommentCollector<'a>(Vec<&'a ast::Comment>);

impl<'ast> Visitor<'ast> for CommentCollector<'ast> {
    type BreakTy = ();

    fn visit_comment(
        &mut self,
        comment: &'ast ast::Comment,
    ) -> std::ops::ControlFlow<Self::BreakTy> {
        self.0.push(comment);
        std::ops::ControlFlow::Continue(())
    }
}
