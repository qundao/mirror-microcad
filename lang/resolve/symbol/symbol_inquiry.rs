/// Symbol doc retrieved from markdown.
struct SymbolDoc {
    summary: String,
    description: String,
}

/// Infos about parameters (Might be directly retrieved via ParameterList or ParameterValueList?)
struct ParameterInfo {
    name: String,
    ty: String,
}

struct SymbolInfo {
    doc: SymbolDoc,
    parameters: Vec<ParameterInfo>,
}

trait SymbolInquiry {
    fn get_symbol_info() -> SymbolInfo;
}
