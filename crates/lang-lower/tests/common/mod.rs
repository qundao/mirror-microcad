use microcad_lang_base::{CompilationResult, MICROCAD_EXTENSION, Source};
use microcad_lang_lower::{self as lower, LowerContext, LowerError, ir};
use microcad_lang_parse as parse;

/// Get intermediate representation and diagnostics.
#[allow(unused)]
pub fn ir_from_source(source: &Source) -> CompilationResult<lower::Ir, LowerError> {
    let ast = parse::parse(source).expect("No parse errors").0;
    let mut context = LowerContext::from(source);
    let (mut ir, errors) = lower::lower(&mut context, &ast)?;

    use microcad_lang_lower::ir::visitor::VisitorMut;
    ir::visitor::MakeHumanReadable::new(&context).visit(&mut ir);

    Ok((ir, errors))
}

#[allow(unused)]
pub fn source_from_test_file(name: &str) -> Source {
    Source::load(format!("tests/test_cases/{name}.{}", MICROCAD_EXTENSION)).expect("No error")
}
