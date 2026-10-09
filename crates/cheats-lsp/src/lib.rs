use cheats_core::{CheatSheet, SearchRequest};
use cheats_data::load_dir;
use cheats_search::search;
use std::{path::PathBuf, sync::Arc};
use tower_lsp_server::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
    sheets: Arc<Vec<CheatSheet>>,
}

impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo { name: "rust-cheats".into(), version: Some(env!("CARGO_PKG_VERSION").into()) }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
                completion_provider: Some(CompletionOptions { trigger_characters: Some(vec!["/".into(), ".".into()]), ..Default::default() }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        let _ = self.client.log_message(MessageType::INFO, "rust-cheats LSP ready (local data only)").await;
    }

    async fn shutdown(&self) -> Result<()> { Ok(()) }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let prefix = params.context.as_ref().map(|_| "").unwrap_or("");
        let items = self.sheets.iter()
            .filter(|s| s.id.starts_with(prefix) || s.title.to_lowercase().contains(&prefix.to_lowercase()))
            .take(100)
            .map(|s| CompletionItem {
                label: s.title.clone(),
                detail: Some(format!("{} · {}", s.language, s.id)),
                insert_text: Some(s.id.clone()),
                kind: Some(CompletionItemKind::REFERENCE),
                ..Default::default()
            }).collect::<Vec<_>>();
        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let path = uri.path().to_string();
        let word = path.rsplit('/').next().unwrap_or_default();
        let request = SearchRequest { query: word.to_string(), language: None, limit: 1 };
        let result = search(&self.sheets, &request).into_iter().next();
        Ok(result.map(|r| Hover {
            contents: HoverContents::Scalar(MarkedString::String(format!("**{}**\n\n{}\n\n`{}`", r.title, r.description, r.id))),
            range: None,
        }))
    }
}

pub async fn run(data_dir: PathBuf) -> anyhow::Result<()> {
    let sheets = load_dir(data_dir)?;
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(|client| Backend { client, sheets: Arc::new(sheets.clone()) });
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
