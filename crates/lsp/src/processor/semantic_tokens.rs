// Copyright © 2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_lang::{
    syntax::{
        Argument, ArrayExpression, ArrayExpressionInner, Assignment, AssignmentStatement,
        Attribute, AttributeCommand, AttributeList, Call, Expression, ExpressionStatement,
        FormatExpression, FormatString, FormatStringInner, FunctionDefinition, FunctionSignature,
        Identifiable, Identifier, IfStatement, InitDefinition, InnerDocComment, Literal, Marker,
        MethodCall, ModuleDefinition, ParameterList, QualifiedName, Qualifier, RangeExpression,
        ReturnStatement, SourceFile, Statement, StatementList, TupleExpression, TypeAnnotation,
        UseDeclaration, UseStatement, WorkbenchDefinition,
    },
    ty::{Ty, Type},
};
use microcad_lang_base::{Refer, SrcRef, SrcReferrer};

use tower_lsp::lsp_types::{SemanticToken, SemanticTokenModifier, SemanticTokenType, Url};

use crate::processor::src_ref_to_lsp_range;

pub(crate) struct TokenContext {
    source_file: Rc<SourceFile>,
    last_line: u32,
    last_col: u32,
}

impl TokenContext {
    pub(crate) fn new(url: &Url) -> miette::Result<Self> {
        let src_path = url
            .to_file_path()
            .map_err(|_| miette::miette!("Error converting {url} to file path."))?;
        let source_file = SourceFile::load(src_path)
            .map_err(|e| miette::miette!("Error loading source file: {e}"))?;
        Ok(Self {
            source_file,
            last_line: 0,
            last_col: 0,
        })
    }

    pub(crate) fn parse_semantic_tokens(&mut self) -> miette::Result<Vec<SemanticToken>> {
        let stmts = self.source_file.statements.clone();
        self.parse_statement_list(&stmts)
    }

    fn parse_statement(&mut self, stmt: &Statement) -> miette::Result<Vec<SemanticToken>> {
        match stmt {
            Statement::Workbench(workbench_definition) => {
                self.parse_workbench(workbench_definition)
            }
            Statement::Assignment(assignment) => self.parse_assignment_stmt(assignment),
            Statement::InnerDocComment(comment) => self.parse_comment(comment),
            Statement::Expression(expression) => self.parse_expression_stmt(expression),
            Statement::Use(use_statement) => self.parse_use(use_statement),
            Statement::If(if_statement) => self.parse_if_stmt(if_statement),
            Statement::Module(module_definition) => self.parse_module_stmt(module_definition),
            Statement::Function(function_definition) => {
                self.parse_function_stmt(function_definition)
            }
            Statement::Init(init_definition) => self.parse_init_stmt(init_definition),
            Statement::Return(return_statement) => self.parse_return_stmt(return_statement),
            Statement::InnerAttribute(attribute) => self.parse_inner_attribute(attribute),
        }
    }

    fn parse_workbench(
        &mut self,
        workbench: &WorkbenchDefinition,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(
            workbench
                .doc
                .clone()
                .map(|doc| {
                    self.output_semantic_token(doc.src_ref(), SemanticTokenType::COMMENT, &[])
                })
                .transpose()?,
        );
        tokens.extend(self.parse_attribute_list(&workbench.attribute_list)?);
        tokens.extend([
            self.output_semantic_token(
                workbench.keyword_ref.clone(),
                SemanticTokenType::KEYWORD,
                &[],
            )?,
            self.output_semantic_token(
                workbench.src_ref(), // ID src ref
                SemanticTokenType::FUNCTION,
                &[],
            )?,
        ]);
        tokens.extend(self.parse_parameter_list(&workbench.plan)?);
        tokens.extend(self.parse_statement_list(&workbench.body)?);
        Ok(tokens)
    }

    fn parse_attribute_list(
        &mut self,
        attr_list: &AttributeList,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        for attribute in attr_list.iter() {
            // tokens.push(self.output_semantic_token(
            //     attribute.src_ref(), // Attribute src ref
            //     SemanticTokenType::VARIABLE,
            //     &[SemanticTokenModifier::DEFINITION],
            // )?);
            for command in &attribute.commands {
                tokens.extend(self.parse_attribute_command(command)?);
            }
        }
        Ok(tokens)
    }

    fn parse_parameter_list(&mut self, plan: &ParameterList) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        for param in plan.iter() {
            tokens.extend(
                self.output_semantic_token(param.src_ref(), SemanticTokenType::PARAMETER, &[])
                    .ok(),
            );
            tokens.extend(
                self.output_semantic_token(
                    param.id_ref().src_ref(),
                    SemanticTokenType::PARAMETER,
                    &[],
                )
                .ok(),
            );
            if let Some(specified_type) = &param.specified_type {
                tokens.push(self.output_semantic_token(
                    specified_type.src_ref(),
                    SemanticTokenType::TYPE,
                    &[],
                )?);
            }
            if let Some(default_value) = &param.default_value {
                tokens.extend(self.parse_expression(default_value)?);
            }
        }
        Ok(tokens)
    }

    fn parse_statement_list(
        &mut self,
        statements: &StatementList,
    ) -> miette::Result<Vec<SemanticToken>> {
        let tokens = statements
            .iter()
            .map(|stmt| self.parse_statement(stmt))
            .flat_map(Result::ok)
            .flatten()
            .collect::<Vec<_>>();
        Ok(tokens)
    }

    fn parse_literal(&mut self, lit: &Literal) -> miette::Result<Vec<SemanticToken>> {
        Ok(vec![match lit {
            Literal::Integer(int) => {
                self.output_semantic_token(int.src_ref(), SemanticTokenType::NUMBER, &[])?
            }
            Literal::Number(number) => {
                self.output_semantic_token(
                    number.src_ref(),
                    // SEMANTIC_LENGTH,
                    SemanticTokenType::NUMBER,
                    &[],
                )?
            }
            Literal::Bool(bool) => {
                self.output_semantic_token(bool.src_ref(), SemanticTokenType::KEYWORD, &[])?
            }
        }])
    }

    fn parse_expression(&mut self, expr: &Expression) -> miette::Result<Vec<SemanticToken>> {
        let ret = match expr {
            Expression::Invalid => miette::bail!("Invalid expression"),
            Expression::Literal(literal) => self.parse_literal(literal)?,
            Expression::UnaryOp { op, rhs, src_ref } => self.parse_unary_op(op, rhs, src_ref)?,
            Expression::BinaryOp {
                lhs,
                op,
                rhs,
                src_ref,
            } => self.parse_binary_op(lhs, op, rhs, src_ref)?,
            Expression::QualifiedName(qualified_name) => {
                self.parse_qualified_name(qualified_name, SemanticTokenType::VARIABLE)?
            }
            Expression::MethodCall(expression, method_call, src_ref) => {
                self.parse_method_call(expression, method_call, src_ref)?
            }
            Expression::Call(call) => self.parse_call(call)?,
            Expression::TupleExpression(tuple) => self.parse_tuple_expression(tuple)?,
            Expression::ArrayExpression(array_expr) => self.parse_array_expression(array_expr)?,
            Expression::Body(body) => self.parse_statement_list(body)?,
            Expression::If(if_stmt) => self.parse_if_stmt(if_stmt)?,
            Expression::Marker(marker) => self.parse_marker(marker)?,
            Expression::FormatString(format_string) => self.parse_format_string(format_string)?,
            Expression::AttributeAccess(expression, identifier, src_ref) => {
                self.parse_attribute_access(expression, identifier, src_ref)?
            }
            Expression::PropertyAccess(expression, identifier, src_ref) => {
                self.parse_property_access(expression, identifier, src_ref)?
            }
            Expression::ArrayElementAccess(expression, expression1, src_ref) => {
                self.parse_array_element_access(expression, expression1, src_ref)?
            }
        };
        Ok(ret)
    }

    fn output_semantic_token(
        &mut self,
        src_ref: SrcRef,
        token_type: SemanticTokenType,
        modifiers: &[SemanticTokenModifier],
    ) -> miette::Result<SemanticToken> {
        let length = src_ref.len() as u32;
        let pos =
            src_ref_to_lsp_range(src_ref).ok_or(miette::miette!("source file not specified"))?;
        let line = pos.start.line;
        let delta_line = line.checked_sub(self.last_line).ok_or_else(|| {
            miette::miette!(
                "Line number overflow: current line {}, last line {}",
                line,
                self.last_line
            )
        })?;
        let char = pos.start.character;
        let delta_start = if line == self.last_line {
            char - self.last_col
        } else {
            char
        };
        dbg!(line, char, length, &token_type, delta_line, delta_start);
        self.last_line = line;
        self.last_col = char;
        Ok(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type: semantic_token_type_index(&token_type),
            token_modifiers_bitset: semantic_token_modifier_bitset(modifiers),
        })
    }

    fn parse_assignment(&mut self, assignment: &Assignment) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(
            assignment
                .doc
                .as_ref()
                .map(|doc| {
                    self.output_semantic_token(doc.src_ref(), SemanticTokenType::COMMENT, &[])
                })
                .transpose()?,
        );
        let id_type = match assignment.qualifier() {
            Qualifier::Const => SemanticTokenType::MACRO,
            Qualifier::Value => SemanticTokenType::VARIABLE,
            Qualifier::Prop => SemanticTokenType::PROPERTY,
        };

        tokens.push(self.output_semantic_token(assignment.id_ref().src_ref(), id_type, &[])?);
        if let Some(specified_type) = &assignment.specified_type {
            tokens.push(self.output_semantic_token(
                specified_type.src_ref(),
                SemanticTokenType::TYPE,
                &[SemanticTokenModifier::MODIFICATION],
            )?);
        }

        tokens.extend(self.parse_expression(&assignment.expression)?);
        Ok(tokens)
    }

    fn parse_unary_op(
        &mut self,
        op: &Refer<String>,
        rhs: &Expression,
        _src_ref: &SrcRef,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(op.src_ref(), SemanticTokenType::OPERATOR, &[])?);
        tokens.extend(self.parse_expression(rhs)?);
        Ok(tokens)
    }

    fn parse_binary_op(
        &mut self,
        lhs: &Expression,
        op: &Refer<String>,
        rhs: &Expression,
        _src_ref: &SrcRef,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_expression(lhs)?);
        tokens.push(self.output_semantic_token(op.src_ref(), SemanticTokenType::OPERATOR, &[])?);
        tokens.extend(self.parse_expression(rhs)?);
        Ok(tokens)
    }

    fn parse_qualified_name(
        &mut self,
        qualified_name: &QualifiedName,
        token_type: SemanticTokenType,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            qualified_name.src_ref(),
            token_type.clone(),
            &[],
        )?);
        for id in qualified_name.iter() {
            tokens.push(self.output_semantic_token(id.src_ref(), token_type.clone(), &[])?);
        }
        Ok(tokens)
    }

    fn parse_method_call(
        &mut self,
        expression: &Expression,
        method_call: &MethodCall,
        _src_ref: &SrcRef,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_expression(expression)?);
        tokens.push(self.output_semantic_token(
            method_call.name.src_ref(),
            SemanticTokenType::FUNCTION,
            &[],
        )?);
        for arg in method_call.argument_list.iter() {
            tokens.extend(self.parse_argument(arg)?);
        }
        Ok(tokens)
    }

    fn parse_argument(&mut self, arg: &Argument) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        if let Some(id) = &arg.id {
            tokens.push(self.output_semantic_token(
                id.src_ref(),
                SemanticTokenType::PROPERTY,
                &[],
            )?);
        }
        tokens.extend(self.parse_expression(&arg.expression)?);
        Ok(tokens)
    }

    fn parse_call(&mut self, call: &Call) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_qualified_name(&call.name, SemanticTokenType::FUNCTION)?);
        for arg in call.argument_list.iter() {
            tokens.extend(self.parse_argument(arg)?);
        }
        Ok(tokens)
    }

    fn parse_comment(&mut self, comment: &InnerDocComment) -> miette::Result<Vec<SemanticToken>> {
        Ok(vec![self.output_semantic_token(
            comment.src_ref(),
            SemanticTokenType::COMMENT,
            &[],
        )?])
    }

    fn parse_use(&mut self, use_statement: &UseStatement) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            use_statement.keyword_ref.clone(),
            SemanticTokenType::KEYWORD,
            &[],
        )?);
        tokens.extend(self.parse_use_decl(&use_statement.decl)?);
        Ok(tokens)
    }

    fn parse_use_decl(&mut self, decl: &UseDeclaration) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        match decl {
            UseDeclaration::UseAll(name) | UseDeclaration::Use(name) => {
                tokens.extend(self.parse_qualified_name(name, SemanticTokenType::NAMESPACE)?);
            }
            UseDeclaration::UseAs(name, id) => {
                tokens.extend(self.parse_qualified_name(name, SemanticTokenType::NAMESPACE)?);
                tokens.push(self.output_semantic_token(
                    id.src_ref(),
                    SemanticTokenType::NAMESPACE,
                    &[],
                )?);
            }
        }
        Ok(tokens)
    }

    fn parse_tuple_expression(
        &mut self,
        tuple: &TupleExpression,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        for arg in tuple.args.iter() {
            tokens.extend(self.parse_argument(arg)?);
        }
        Ok(tokens)
    }

    fn parse_array_expression(
        &mut self,
        array_expr: &ArrayExpression,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        match &array_expr.inner {
            ArrayExpressionInner::List(expressions) => tokens.extend(
                expressions
                    .iter()
                    .map(|e| self.parse_expression(e))
                    .flat_map(Result::ok)
                    .flatten(),
            ),
            ArrayExpressionInner::Range(range_expression) => {
                tokens.extend(self.parse_range_expression(range_expression)?);
            }
        }
        Ok(tokens)
    }

    fn parse_range_expression(
        &mut self,
        range_expression: &RangeExpression,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_expression(&range_expression.first)?);
        tokens.extend(self.parse_expression(&range_expression.last)?);
        Ok(tokens)
    }

    fn parse_marker(&mut self, marker: &Marker) -> miette::Result<Vec<SemanticToken>> {
        Ok(vec![self.output_semantic_token(
            marker.src_ref(),
            SemanticTokenType::EVENT,
            &[],
        )?])
    }

    fn parse_attribute_command(
        &mut self,
        command: &AttributeCommand,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        match command {
            AttributeCommand::Ident(name) => {
                tokens.push(self.output_semantic_token(
                    name.src_ref(),
                    SemanticTokenType::PROPERTY,
                    &[],
                )?);
            }
            AttributeCommand::Call(call) => {
                tokens.extend(self.parse_call(call)?);
            }
            AttributeCommand::Assigment { name, value, .. } => {
                tokens.push(self.output_semantic_token(
                    name.src_ref(),
                    SemanticTokenType::PROPERTY,
                    &[],
                )?);
                tokens.extend(self.parse_expression(value)?);
            }
        }
        Ok(tokens)
    }

    fn parse_format_string(
        &mut self,
        format_string: &FormatString,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            format_string.0.src_ref(),
            SemanticTokenType::STRING,
            &[],
        )?);
        for elem in &*format_string.0 {
            match elem {
                FormatStringInner::String(_) => tokens.push(self.output_semantic_token(
                    elem.src_ref(),
                    SemanticTokenType::STRING,
                    &[],
                )?),
                FormatStringInner::FormatExpression(expr) => {
                    tokens.extend(self.parse_format_expression(expr)?)
                }
            }
        }
        Ok(tokens)
    }

    fn parse_format_expression(
        &mut self,
        expr: &FormatExpression,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_expression(&expr.expression)?);
        if let Some(spec) = &expr.spec {
            tokens.push(self.output_semantic_token(
                spec.src_ref(),
                SemanticTokenType::STRING,
                &[],
            )?);
        }
        Ok(tokens)
    }

    fn parse_attribute_access(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        src_ref: &SrcRef,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            src_ref.clone(),
            SemanticTokenType::PROPERTY,
            &[],
        )?);
        tokens.extend(self.parse_expression(expression)?);
        tokens.push(self.output_semantic_token(
            identifier.src_ref(),
            SemanticTokenType::PROPERTY,
            &[],
        )?);
        Ok(tokens)
    }

    fn parse_property_access(
        &mut self,
        expression: &Expression,
        identifier: &Identifier,
        _src_ref: &SrcRef,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_expression(expression)?);
        tokens.push(self.output_semantic_token(
            identifier.src_ref(),
            SemanticTokenType::PROPERTY,
            &[],
        )?);

        Ok(tokens)
    }

    fn parse_array_element_access(
        &mut self,
        expression: &Expression,
        expression1: &Expression,
        _src_ref: &SrcRef,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_expression(expression)?);
        tokens.extend(self.parse_expression(expression1)?);
        Ok(tokens)
    }

    fn parse_function_signature(
        &mut self,
        signature: &FunctionSignature,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];

        tokens.extend(self.parse_parameter_list(&signature.parameters)?);
        if let Some(return_type) = &signature.return_type {
            tokens.extend(self.parse_type_annotation(return_type)?);
        }

        Ok(tokens)
    }

    fn parse_type_annotation(
        &mut self,
        return_type: &TypeAnnotation,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        match return_type.ty() {
            Type::Array(_) => tokens.push(self.output_semantic_token(
                return_type.0.src_ref(),
                SemanticTokenType::TYPE,
                &[],
            )?),
            _ => tokens.push(self.output_semantic_token(
                return_type.src_ref(),
                SemanticTokenType::TYPE,
                &[],
            )?),
        }
        Ok(tokens)
    }

    fn parse_inner_attribute(
        &mut self,
        attribute: &Attribute,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        for command in &attribute.commands {
            tokens.extend(self.parse_attribute_command(command)?);
        }
        Ok(tokens)
    }
}

// Statement parsers
impl TokenContext {
    fn parse_expression_stmt(
        &mut self,
        expression: &ExpressionStatement,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_attribute_list(&expression.attribute_list)?);
        tokens.extend(self.parse_expression(&expression.expression)?);
        Ok(tokens)
    }

    fn parse_if_stmt(&mut self, if_stmt: &IfStatement) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            if_stmt.if_ref.clone(),
            SemanticTokenType::KEYWORD,
            &[],
        )?);
        tokens.extend(self.parse_expression(&if_stmt.cond)?);
        tokens.extend(self.parse_statement_list(&if_stmt.body)?);
        if let Some(next_if) = &if_stmt.next_if
            && let Some(next_if_ref) = &if_stmt.next_if_ref
        {
            tokens.push(self.output_semantic_token(
                next_if_ref.clone(),
                SemanticTokenType::KEYWORD,
                &[],
            )?);
            tokens.extend(self.parse_if_stmt(next_if)?);
        }
        if let Some(body) = &if_stmt.body_else
            && let Some(else_ref) = &if_stmt.else_ref
        {
            tokens.push(self.output_semantic_token(
                else_ref.clone(),
                SemanticTokenType::KEYWORD,
                &[],
            )?);
            tokens.extend(self.parse_statement_list(body)?);
        }
        Ok(tokens)
    }

    fn parse_module_stmt(
        &mut self,
        module_definition: &ModuleDefinition,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            module_definition.keyword_ref.clone(),
            SemanticTokenType::KEYWORD,
            &[],
        )?);
        tokens.push(self.output_semantic_token(
            module_definition.id_ref().src_ref(),
            SemanticTokenType::NAMESPACE,
            &[SemanticTokenModifier::DEFINITION],
        )?);
        if let Some(body) = &module_definition.body {
            tokens.extend(self.parse_statement_list(body)?);
        }
        Ok(tokens)
    }

    fn parse_function_stmt(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        if let Some(doc) = &function_definition.doc {
            tokens.push(self.output_semantic_token(
                doc.src_ref(),
                SemanticTokenType::COMMENT,
                &[],
            )?);
        }
        tokens.push(self.output_semantic_token(
            function_definition.keyword_ref.clone(),
            SemanticTokenType::KEYWORD,
            &[],
        )?);
        tokens.push(self.output_semantic_token(
            function_definition.id_ref().src_ref(),
            SemanticTokenType::FUNCTION,
            &[SemanticTokenModifier::DEFINITION],
        )?);
        tokens.extend(self.parse_function_signature(&function_definition.signature)?);
        tokens.extend(self.parse_statement_list(&function_definition.body)?);
        Ok(tokens)
    }

    fn parse_init_stmt(
        &mut self,
        init_definition: &InitDefinition,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        if let Some(doc) = &init_definition.doc {
            tokens.push(self.output_semantic_token(
                doc.src_ref(),
                SemanticTokenType::COMMENT,
                &[],
            )?);
        }
        tokens.push(self.output_semantic_token(
            init_definition.keyword_ref.clone(),
            SemanticTokenType::KEYWORD,
            &[],
        )?);
        tokens.extend(self.parse_parameter_list(&init_definition.parameters)?);
        tokens.extend(self.parse_statement_list(&init_definition.body)?);
        Ok(tokens)
    }

    fn parse_assignment_stmt(
        &mut self,
        assignment: &AssignmentStatement,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.extend(self.parse_attribute_list(&assignment.attribute_list)?);
        tokens.extend(self.parse_assignment(&assignment.assignment)?);

        Ok(tokens)
    }

    fn parse_return_stmt(
        &mut self,
        return_statement: &ReturnStatement,
    ) -> miette::Result<Vec<SemanticToken>> {
        let mut tokens = vec![];
        tokens.push(self.output_semantic_token(
            return_statement.keyword_ref.clone(),
            SemanticTokenType::KEYWORD,
            &[],
        )?);
        if let Some(expr) = &return_statement.result {
            tokens.extend(self.parse_expression(expr)?);
        }
        Ok(tokens)
    }
}

const SEMANTIC_LENGTH: SemanticTokenType = SemanticTokenType::new("length");

pub const LEGEND_TYPES: &[SemanticTokenType] = &[
    SemanticTokenType::NAMESPACE,
    SemanticTokenType::TYPE,
    SemanticTokenType::CLASS,
    SemanticTokenType::ENUM,
    SemanticTokenType::INTERFACE,
    SemanticTokenType::STRUCT,
    SemanticTokenType::TYPE_PARAMETER,
    SemanticTokenType::PARAMETER,
    SemanticTokenType::VARIABLE,
    SemanticTokenType::PROPERTY,
    SemanticTokenType::ENUM_MEMBER,
    SemanticTokenType::EVENT,
    SemanticTokenType::FUNCTION,
    SemanticTokenType::METHOD,
    SemanticTokenType::MACRO,
    SemanticTokenType::KEYWORD,
    SemanticTokenType::MODIFIER,
    SemanticTokenType::COMMENT,
    SemanticTokenType::STRING,
    SemanticTokenType::NUMBER,
    SEMANTIC_LENGTH,
    SemanticTokenType::REGEXP,
    SemanticTokenType::OPERATOR,
    SemanticTokenType::DECORATOR,
];

pub const LEGEND_MODIFIERS: &[SemanticTokenModifier] = &[
    SemanticTokenModifier::DECLARATION,
    SemanticTokenModifier::DEFINITION,
    SemanticTokenModifier::READONLY,
    SemanticTokenModifier::STATIC,
    SemanticTokenModifier::DEPRECATED,
    SemanticTokenModifier::ABSTRACT,
    SemanticTokenModifier::ASYNC,
    SemanticTokenModifier::MODIFICATION,
    SemanticTokenModifier::DOCUMENTATION,
    SemanticTokenModifier::DEFAULT_LIBRARY,
];

fn semantic_token_type_index(token_type: &SemanticTokenType) -> u32 {
    LEGEND_TYPES
        .iter()
        .position(|t| t == token_type)
        .expect("Token type not found in legend") as u32
}

fn semantic_token_modifier_index(token_modifier: &SemanticTokenModifier) -> u32 {
    1 << LEGEND_MODIFIERS
        .iter()
        .position(|t| t == token_modifier)
        .expect("Token modifier not found in legend") as u32
}

fn semantic_token_modifier_bitset(token_modifiers: &[SemanticTokenModifier]) -> u32 {
    token_modifiers
        .iter()
        .map(semantic_token_modifier_index)
        .reduce(|a, b| a | b)
        .unwrap_or(0)
}
