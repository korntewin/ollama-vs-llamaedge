use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Serialize)]
pub struct ChatRequest<'a> {
    pub model: &'a str,
    pub messages: Vec<ChatMessage<'a>>,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub usage: Usage,
}

#[derive(Debug, Deserialize)]
pub struct Usage {
    pub completion_tokens: u64,
}
