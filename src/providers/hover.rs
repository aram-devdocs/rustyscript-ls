use tower_lsp::lsp_types::{Position, TextDocumentIdentifier};

pub fn get_hover(_uri: &TextDocumentIdentifier, _position: &Position) -> String {
    "RustyScript-LS Hover Information\n\n- Type: `DummyType`\n- Documentation: Example hover content".to_string()
}
