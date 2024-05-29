use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc::Result, Client, LanguageServer, LspService, Server};

macro_rules! code_action {
    ($title:expr) => {
        CodeActionOrCommand::CodeAction(CodeAction {
            title: ($title).into(),
            kind: Some(CodeActionKind::REFACTOR),
            diagnostics: None,
            edit: None,
            command: None,
            is_preferred: Some(false),
            disabled: None,
            data: None,
        })
    };
}

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Default::default(),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::LOG, "o(≧∇≦)o Good morning!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.client
            .log_message(MessageType::LOG, "_(ᴗ˳ᴗ)_ Good night!")
            .await;
        Ok(())
    }

    async fn code_action(&self, _: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        let caa = CodeActionOrCommand::CodeAction(CodeAction {
            title: ("Test Edit").into(),
            kind: Some(CodeActionKind::REFACTOR),
            diagnostics: None,
            edit: Some(WorkspaceEdit {
                changes: None,
                document_changes: None,
                change_annotations: None,
            }),
            command: None,
            is_preferred: Some(false),
            disabled: None,
            data: None,
        });

        let ca: CodeActionResponse = vec![
            caa,
            code_action!("Insert meaning of life before the current symbol"),
            code_action!("Generate boyfriend"),
            code_action!("Finish Game"),
            code_action!("Call ThePrimeagen to react to yet another new language"),
            code_action!("Call Theo but he won't like it cuz it's not JS"),
            code_action!("Stop making jokes and begin making stuff"),
        ];

        Ok(Some(ca))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
