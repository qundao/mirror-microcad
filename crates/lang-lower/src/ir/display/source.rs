use microcad_lang_base::DisplayOneLine;

use crate::ir;

impl std::fmt::Display for ir::SourceStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO Display attributes and exports
        match &self.name {
            Some(name) => {
                write!(f, "{name}: {ty} = ", ty = self.ty)?;
            }
            None => {}
        };

        write!(f, "{}", self.expression)
    }
}

impl DisplayOneLine for ir::SourceStatement {}

impl std::fmt::Display for ir::Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        super::fmt_statements(&self.statements, f)
    }
}
