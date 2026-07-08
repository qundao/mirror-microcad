// Copyright © 2024-2026 The µcad authors <info@microcad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::hash::HashSet;
use microcad_lang_base::{
    Diag, DiagHandler, DiagResult, Diagnostic, FormatTree, GetSourceLocInfoByHash, HashId, Output,
    PushDiag, SourceLocInfo, SrcReferrer, TreeDisplay, TreeState,
};

use crate::{
    builtin::*,
    eval::*,
    lower::{SingleIdentifier, ir},
    model::*,
    symbol::SymbolDef,
};

/// *Context* for *evaluation* of a resolved µcad file.
///
/// The context is used to store the current state of the evaluation.
pub struct EvalContext {
    /// Symbol table
    pub root: Symbol,
    /// Source cache
    sources: Sources,
    /// Stack of currently opened scopes with symbols while evaluation.
    pub(super) stack: Stack,
    /// Output channel for [__builtin::print].
    output: Box<dyn Output>,
    /// Exporter registry.
    exporters: ExporterRegistry,
    /// Importer registry.
    importers: ImporterRegistry,
    /// Diagnostics handler.
    pub diag: DiagHandler,
}

impl EvalContext {
    /// Create a new context from a resolved symbol table.
    pub fn new(
        resolve_context: ResolveContext,
        output: Box<dyn Output>,
        exporters: ExporterRegistry,
        importers: ImporterRegistry,
    ) -> Self {
        log::debug!("Creating evaluation context");

        Self {
            root: resolve_context.root,
            sources: resolve_context.sources,
            diag: resolve_context.diag,
            output,
            exporters,
            importers,
            stack: Stack::default(),
        }
    }

    /// Current symbol, panics if there no current symbol.
    pub(crate) fn current_symbol(&self) -> Option<Symbol> {
        self.stack.current_symbol()
    }

    /// Create a new context from a source file.
    pub fn from_source(
        root: std::rc::Rc<ir::Source>,
        builtin: Option<Symbol>,
        search_paths: Vec<std::path::PathBuf>,
        output: Box<dyn Output>,
        exporters: ExporterRegistry,
        importers: ImporterRegistry,
    ) -> EvalResult<Self> {
        Ok(Self::new(
            ResolveContext::create(root, search_paths, builtin, DiagHandler::default())?,
            output,
            exporters,
            importers,
        ))
    }

    /// Access captured output.
    pub fn output(&self) -> Option<String> {
        self.output.output()
    }

    /// Print for `__builtin::print`.
    pub fn print(&mut self, what: String) {
        self.output.print(what).expect("could not write to output");
    }

    /// Evaluate context into a value.
    pub fn eval(&mut self) -> EvalResult<Model> {
        if self.diag.error_count() > 0 {
            log::error!("Aborting evaluation because of prior resolve errors!");
            return Err(EvalError::ResolveFailed.into());
        }
        let model: Model = self.sources.root().eval(self)?;
        log::trace!("Post-evaluation context:\n{self:?}");
        log::trace!("Evaluated Model:\n{}", FormatTree(&model));

        let unused = self
            .root
            .unused_private()
            .iter()
            .map(|symbol| {
                (
                    match self.sources.get_code(&symbol) {
                        Ok(id) => id,
                        Err(_) => symbol.id().to_string(),
                    },
                    symbol.src_ref(),
                )
            })
            // intermediate hasp storage to avoid duplicates
            .collect::<indexmap::IndexMap<_, _>>();

        unused.into_iter().try_for_each(|(id, src_ref)| {
            self.warning(&src_ref, EvalError::UnusedGlobalSymbol(id))
        })?;

        Ok(model)
    }

    /// Run the closure `f` within the given `stack_frame`.
    pub(super) fn scope<T>(
        &mut self,
        stack_frame: StackFrame,
        f: impl FnOnce(&mut EvalContext) -> T,
    ) -> T {
        self.open(stack_frame);
        let result = f(self);
        let mut unused: Vec<_> = if let Some(frame) = &self.stack.current_frame() {
            if let Some(locals) = frame.locals() {
                locals
                    .iter()
                    .filter(|(_, symbol)| !symbol.is_used())
                    .filter(|(id, _)| !id.ignore())
                    .filter(|(_, symbol)| !symbol.src_ref().is_none())
                    .map(|(id, _)| id.clone())
                    .collect()
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        unused.sort();

        unused
            .iter()
            .try_for_each(|id| self.warning(id, EvalError::UnusedLocal(id.clone())))
            .expect("diag error");

        self.close();
        result
    }

    /// All registered exporters.
    pub fn exporters(&self) -> &ExporterRegistry {
        &self.exporters
    }

    /// Return search paths of this context.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        self.sources.search_paths()
    }

    /// Get property from current model.
    pub(super) fn get_property(&self, id: &Identifier) -> EvalResult<Value> {
        match self.get_model() {
            Ok(model) => {
                if let Some(value) = model.get_property(id) {
                    Ok(value.clone())
                } else {
                    Err(EvalError::PropertyNotFound(id.clone()).into())
                }
            }
            Err(err) => Err(err),
        }
    }

    /// Initialize a property.
    ///
    /// Returns error if there is no model or the property has been initialized before.
    pub(super) fn init_property(&self, id: Identifier, value: Value) -> EvalResult<()> {
        match self.get_model() {
            Ok(model) => {
                if let Some(previous_value) = model.borrow_mut().set_property(id.clone(), value) {
                    if !previous_value.is_invalid() {
                        return Err(EvalError::ValueAlreadyDefined {
                            location: id.src_ref(),
                            name: id.clone(),
                            value: previous_value.to_string(),
                            previous_location: id.src_ref(),
                        }
                        .into());
                    }
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    /// Return if the current frame is an init frame.
    pub(super) fn is_init(&mut self) -> bool {
        matches!(self.stack.current_frame(), Some(StackFrame::Init(_)))
    }

    /// Lookup a property by qualified name.
    fn lookup_property(&self, name: &ir::QualifiedName) -> EvalResult<Symbol> {
        log::trace!(
            "{lookup} for property {name:?}",
            lookup = microcad_lang_base::mark!(LOOKUP)
        );

        if self.stack.current_call_name().is_some() {
            if let Some(id) = name.single_identifier() {
                match self.get_property(id) {
                    Ok(value) => {
                        log::trace!(
                            "{found} property '{name:?}'",
                            found = microcad_lang_base::mark!(FOUND)
                        );
                        return Ok(Symbol::new(SymbolDef::Value(id.clone(), value), None));
                    }
                    Err(err) => return Err(err),
                }
            }
        }
        log::trace!(
            "{not_found} Property '{name:?}'",
            not_found = microcad_lang_base::mark!(NOT_FOUND)
        );
        Err(EvalError::NoPropertyId(name.clone()).into())
    }

    fn lookup_workbench(
        &self,
        name: &ir::QualifiedName,
        target: LookupTarget,
    ) -> ResolveResult<Symbol> {
        if let Some(workbench) = &self.stack.current_call_name() {
            log::trace!(
                "{lookup} for symbol '{name:?}' in current workbench '{workbench:?}'",
                lookup = microcad_lang_base::mark!(LOOKUP)
            );
            match self.root.lookup_within_name(name, workbench, target) {
                Ok(symbol) => {
                    log::trace!(
                        "{found} symbol in current module: {symbol:?}",
                        found = microcad_lang_base::mark!(FOUND),
                    );
                    Ok(symbol)
                }
                Err(err) => {
                    log::trace!(
                        "{not_found} symbol '{name:?}': {err}",
                        not_found = microcad_lang_base::mark!(NOT_FOUND)
                    );
                    Err(err)
                }
            }
        } else {
            log::trace!(
                "{not_found} No current workbench",
                not_found = microcad_lang_base::mark!(NOT_FOUND)
            );
            Err(ResolveError::SymbolNotFound(name.clone()))
        }
    }

    fn lookup_within(
        &self,
        name: &ir::QualifiedName,
        target: LookupTarget,
    ) -> ResolveResult<Symbol> {
        self.root.lookup_within(
            name,
            &self.root.search(&self.stack.current_module_name(), false)?,
            target,
        )
    }
}

impl Locals for EvalContext {
    fn set_local_value(&mut self, id: Identifier, value: Value) -> EvalResult<()> {
        self.stack.set_local_value(id, value)
    }

    fn get_local_value(&self, id: &Identifier) -> EvalResult<Value> {
        self.stack.get_local_value(id)
    }

    fn open(&mut self, frame: StackFrame) {
        self.stack.open(frame);
    }

    fn close(&mut self) -> StackFrame {
        self.stack.close()
    }

    fn fetch_symbol(&self, id: &Identifier) -> EvalResult<Symbol> {
        self.stack.fetch_symbol(id)
    }

    fn get_model(&self) -> EvalResult<Model> {
        self.stack.get_model()
    }
}

impl ImporterRegistryAccess for EvalContext {
    type Error = Box<EvalError>;

    fn import(
        &mut self,
        arg_map: &Tuple,
        search_paths: &[std::path::PathBuf],
    ) -> Result<Value, Self::Error> {
        match self.importers.import(arg_map, search_paths) {
            Ok(value) => Ok(value),
            Err(err) => {
                self.error(arg_map, err)?;
                Ok(Value::None)
            }
        }
    }
}

impl ExporterAccess for EvalContext {
    fn exporter_by_id(&self, id: &crate::Id) -> Result<std::rc::Rc<dyn Exporter>, ExportError> {
        self.exporters.exporter_by_id(id)
    }

    fn exporter_by_filename(
        &self,
        filename: &std::path::Path,
    ) -> Result<std::rc::Rc<dyn Exporter>, ExportError> {
        self.exporters.exporter_by_filename(filename)
    }
}
