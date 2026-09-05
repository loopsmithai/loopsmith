use serde::Serialize;

/// Standard response envelope for all smith-agent operations.
/// Every node in the command graph — group or leaf — returns this shape.
///
/// Exit codes determine which fields are populated:
/// - Exit 0 (success): summary + result + next + related
/// - Exit 1 (failure): summary + error (with recovery)
/// - Exit 2 (discovery): summary + next (children from graph)
#[derive(Debug, Serialize)]
pub struct Envelope {
    /// One-sentence summary of what happened.
    pub summary: String,

    /// Operation result data — only on exit 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    /// Suggested next steps — runnable commands with real values.
    pub next: Vec<String>,

    /// Related operations the agent might not know about.
    pub related: Vec<String>,

    /// Error details — only on exit 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chain: Vec<String>,
    pub recovery: Vec<String>,
}

/// Exit codes for smith-agent.
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;
pub const EXIT_DISCOVERY: i32 = 2;

impl Envelope {
    /// Build a success envelope (exit 0).
    pub fn success(
        summary: impl Into<String>,
        result: serde_json::Value,
        next: Vec<String>,
        related: Vec<String>,
    ) -> Self {
        Self {
            summary: summary.into(),
            result: Some(result),
            next,
            related,
            error: None,
        }
    }

    /// Build an error envelope (exit 1).
    pub fn error_with_recovery(
        message: impl Into<String>,
        recovery: Vec<String>,
    ) -> Self {
        let msg = message.into();
        Self {
            summary: msg.clone(),
            result: None,
            next: vec![],
            related: vec![],
            error: Some(ErrorDetail {
                message: msg,
                chain: vec![],
                recovery,
            }),
        }
    }

    /// Build an error envelope from an anyhow::Error (exit 1).
    pub fn from_anyhow(err: &anyhow::Error, recovery: Vec<String>) -> Self {
        let mut chain = Vec::new();
        let mut source = err.source();
        while let Some(cause) = source {
            chain.push(format!("{}", cause));
            source = std::error::Error::source(cause);
        }
        Self {
            summary: format!("{}", err),
            result: None,
            next: vec![],
            related: vec![],
            error: Some(ErrorDetail {
                message: format!("{}", err),
                chain,
                recovery,
            }),
        }
    }

    /// Build a discovery envelope (exit 2) — group node, children as next.
    pub fn discovery(summary: impl Into<String>, next: Vec<String>) -> Self {
        Self {
            summary: summary.into(),
            result: None,
            next,
            related: vec![],
            error: None,
        }
    }

    /// The exit code this envelope implies.
    pub fn exit_code(&self) -> i32 {
        if self.error.is_some() {
            EXIT_FAILURE
        } else if self.result.is_some() {
            EXIT_SUCCESS
        } else {
            EXIT_DISCOVERY
        }
    }

    /// Emit as JSON to stdout.
    pub fn print(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            println!("{}", json);
        }
    }
}

use std::error::Error;
