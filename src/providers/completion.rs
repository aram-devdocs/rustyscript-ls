use tower_lsp::lsp_types::{CompletionItem, Position, TextDocumentIdentifier};

pub fn complete(_uri: &TextDocumentIdentifier, _position: &Position) -> Vec<CompletionItem> {
    vec![
        CompletionItem::new_simple("console.log".to_string(), "Log to console".to_string()),
        CompletionItem::new_simple("function".to_string(), "Create function".to_string()),
        CompletionItem::new_simple("interface".to_string(), "Create interface".to_string()),
    ]
}
