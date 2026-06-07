use std::sync::Arc;

use codesift_query::{CodeIntel, QueryHit, StatusResponse};
use rmcp::{
    ErrorData as McpError, ServiceExt,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content},
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
    ServerHandler,
};
use serde::Serialize;

#[derive(Clone)]
pub struct CodesiftServer {
    intel: Arc<CodeIntel>,
    #[allow(dead_code)]
    tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IndexStatusParams {}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetSymbolParams {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub symbol_id: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct FindReferencesParams {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub symbol_id: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetCallersParams {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub symbol_id: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub depth: Option<u32>,
}

#[derive(Serialize)]
struct ToolErrorBody {
    error: ToolError,
}

#[derive(Serialize)]
struct ToolError {
    code: String,
    message: String,
}

impl CodesiftServer {
    pub fn new(intel: CodeIntel) -> Self {
        Self {
            intel: Arc::new(intel),
            tool_router: Self::tool_router(),
        }
    }

    fn json_ok<T: Serialize>(value: &T) -> CallToolResult {
        CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string()),
        )])
    }

    fn tool_error(code: &str, message: impl Into<String>) -> McpError {
        let message = message.into();
        let body = ToolErrorBody {
            error: ToolError {
                code: code.to_string(),
                message: message.clone(),
            },
        };
        McpError::invalid_params(
            serde_json::to_string(&body).unwrap_or(message),
            None,
        )
    }

    fn map_core_err(err: codesift_core::Error) -> McpError {
        let msg = err.to_string();
        let code = if msg.contains("index not found") {
            "INDEX_NOT_FOUND"
        } else if msg.contains("not found") {
            "SYMBOL_NOT_FOUND"
        } else {
            "INVALID_INPUT"
        };
        Self::tool_error(code, msg)
    }
}

#[tool_router]
impl CodesiftServer {
    #[tool(description = "Index freshness and statistics for the workspace")]
    pub fn index_status(
        &self,
        Parameters(_params): Parameters<IndexStatusParams>,
    ) -> Result<CallToolResult, McpError> {
        let status: StatusResponse = self.intel.index_status();
        Ok(Self::json_ok(&status))
    }

    #[tool(description = "Resolve a symbol by name or symbol_id")]
    pub fn get_symbol(
        &self,
        Parameters(params): Parameters<GetSymbolParams>,
    ) -> Result<CallToolResult, McpError> {
        let symbols = self
            .intel
            .resolve_symbol(
                params.name.as_deref(),
                params.symbol_id.as_deref(),
                params.kind.as_deref(),
                params.path.as_deref(),
            )
            .map_err(Self::map_core_err)?;
        if symbols.is_empty() {
            return Err(Self::tool_error(
                "SYMBOL_NOT_FOUND",
                "no matching symbols",
            ));
        }
        Ok(Self::json_ok(&symbols))
    }

    #[tool(description = "Find references to a symbol by name or symbol_id")]
    pub fn find_references(
        &self,
        Parameters(params): Parameters<FindReferencesParams>,
    ) -> Result<CallToolResult, McpError> {
        let hits = self
            .intel
            .find_references(
                params.name.as_deref(),
                params.symbol_id.as_deref(),
                params.kind.as_deref(),
            )
            .map_err(Self::map_core_err)?;
        Ok(Self::json_ok(&hits_json(hits)))
    }

    #[tool(description = "Find callers of a function up to depth (default 1, max 5)")]
    pub fn get_callers(
        &self,
        Parameters(params): Parameters<GetCallersParams>,
    ) -> Result<CallToolResult, McpError> {
        let depth = params.depth.unwrap_or(1).min(5);
        let hits = self
            .intel
            .get_callers(
                params.name.as_deref(),
                params.symbol_id.as_deref(),
                params.kind.as_deref(),
                depth,
            )
            .map_err(Self::map_core_err)?;
        Ok(Self::json_ok(&hits_json(hits)))
    }
}

fn hits_json(hits: Vec<QueryHit>) -> serde_json::Value {
    let total = hits.len();
    serde_json::json!({
        "hits": hits,
        "total": total,
    })
}

#[tool_handler(name = "codesift", version = "0.1.0", instructions = "Structural code intelligence: symbols, references, callers. Use name-first inputs.")]
impl ServerHandler for CodesiftServer {}

pub async fn run_stdio(intel: CodeIntel) -> anyhow::Result<()> {
    let server = CodesiftServer::new(intel);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
