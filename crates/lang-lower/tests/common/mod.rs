use microcad_lang_base::{CompilationResult, MICROCAD_EXTENSION, Source};
use microcad_lang_lower::{self as lower, Desugar, LowerContext, LowerResult, ir};
use microcad_lang_parse as parse;

pub fn desugar(source: &Source) -> LowerResult<ir::desugared::Source> {
    let ast = parse::parse(source).expect("No parse error").0;
    let mut context = LowerContext::from(source);
    ir::desugared::Source::desugar(ast.tree(), &mut context)
}

/// Get intermediate representation and diagnostics.
pub fn ir_from_source(source: &Source) -> CompilationResult<lower::Ir> {
    let ast = parse::parse(source)?.0;
    let mut context = LowerContext::from(source);
    lower::lower(&mut context, &ast)
}

pub fn source_from_test_file(name: &str) -> Source {
    Source::load(format!("tests/test_cases/{name}.{}", MICROCAD_EXTENSION)).expect("No error")
}
