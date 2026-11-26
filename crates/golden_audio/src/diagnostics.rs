use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(all(feature = "analysis", feature = "playback"))]
pub(crate) const REALTIME_PRIORITY_DENIED: &str = "audio_render_realtime_priority_denied";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum DiagnosticSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DiagnosticEvent {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub context: BTreeMap<String, String>,
}

impl DiagnosticEvent {
    #[must_use]
    pub fn new(severity: DiagnosticSeverity, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            context: BTreeMap::new(),
        }
    }

    #[must_use]
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }
}
