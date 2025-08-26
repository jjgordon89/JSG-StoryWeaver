use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Clone, Debug, TS)]
#[ts(export)]
pub struct StreamingEnvelope {
    pub content: String,
    pub is_complete: bool,
    pub token_count: u32,
    pub stream_id: String,
}