use zed_extension_api::{self as zed, LanguageServerId, Result};

struct LSPAIExtension {}

impl zed::Extension for LSPAIExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
        .which("lsp-ai")
        .ok_or_else(|| "Must install lsp-ai cli using `cargo install lsp-ai` or check : https://github.com/SilasMarvin/lsp-ai/wiki/Installation".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let config = serde_json::json!(
            {
                "memory": {
                    "file_store": {}
                  },
                  "models": {
                    "model1": {
                      "type": "ollama",
                      "model": "codegemma"
                    }
                  },
                  "completion": {
                    "model": "model1",
                    "parameters": {
                      "fim": {
                        "start": "<｜fim▁begin｜>",
                        "middle": "<｜fim▁hole｜>",
                        "end": "<｜fim▁end｜>"
                      },
                      "max_context": 2000,
                      "options": {
                        "num_predict": 32
                      }
                    }
                  }
            }
        );

        Ok(Some(config))
    }
}

zed::register_extension!(LSPAIExtension);
