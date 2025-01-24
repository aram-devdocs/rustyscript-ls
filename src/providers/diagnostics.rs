use tower_lsp::lsp_types::Diagnostic;

pub fn analyze(text: &str) -> Vec<Diagnostic> {
    if text.contains("TODO") {
        vec![Diagnostic {
            range: tower_lsp::lsp_types::Range::new(
                tower_lsp::lsp_types::Position::new(0, 0),
                tower_lsp::lsp_types::Position::new(0, 4),
            ),
            message: "Found TODO comment".to_string(),
            severity: Some(tower_lsp::lsp_types::DiagnosticSeverity::WARNING),
            ..Default::default()
        }]
    } else {
        Vec::new()
    }
}
