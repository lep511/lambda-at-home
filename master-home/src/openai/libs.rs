use crate::openai::lib_response::ResponseRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MainRequest {
    Chat(ChatRequest),
    Responses(ResponseRequest),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Chat ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens:  Option<u32>, // For O1 models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(rename = "n")]
    pub n_completion: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub response_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<Value>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedRequest {
    pub model: String,
    pub input: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedResponse {
    pub data: Vec<EmbeddingData>,
    pub model: String,
    pub object: String,
    pub usage: Usage,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingData {
    pub embedding: Vec<f32>,
    pub index: i32,
    pub object: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Platform,
    Developer,
    User,
    Assistant,
    Tool,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: Vec<InputContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_turn: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputContent {
    #[serde(rename = "type")]
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<ImageUrl>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    pub media_type: String,
    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ChatResponse {
    pub choices: Option<Vec<Choice>>,
    pub created: Option<u64>,
    pub id: Option<String>,
    pub model: Option<String>,
    pub object: Option<String>,
    pub system_fingerprint: Option<String>,
    pub usage: Option<Usage>,
    pub chat_history: Option<Vec<Message>>,
    pub service_tier: Option<String>,
    pub error: Option<ErrorDetails>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Choice {
    pub finish_reason: Option<String>,
    pub index: Option<u32>,
    pub logprobs: Option<Value>,
    pub delta: Option<Delta>,
    pub message: Option<ChatMessage>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<String>,
    pub tool_calls: Option<Vec<Value>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub content: Option<String>,
    pub refusal: Option<String>,
    pub role: String,
    pub tool_calls: Option<Vec<Value>>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub completion_tokens: Option<u32>,
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    pub prompt_tokens: Option<u32>,
    pub prompt_tokens_details: Option<PromptTokensDetails>,
    pub total_tokens: Option<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionTokensDetails {
    pub accepted_prediction_tokens: u32,
    pub audio_tokens: u32,
    pub reasoning_tokens: u32,
    pub rejected_prediction_tokens: u32,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromptTokensDetails {
    pub audio_tokens: u32,
    pub cached_tokens: u32,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Errors ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

/// Represents an error response structure from the API
/// 
/// This struct encapsulates the complete error response format,
/// containing both the error type and detailed error information.
///
/// # Fields
/// * `error_type` - The primary classification of the error
/// * `error` - Detailed error information contained in ErrorDetails
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub error: ErrorDetails,
}

/// Contains detailed information about an error
/// 
/// This struct provides specific details about an error occurrence,
/// including the specific error type and a descriptive message.
///
/// # Fields
/// * `code` - Error code
/// * `message` - Detailed description of what went wrong
/// * `param` - Optional - Parameter that caused the error
/// * `error_type` - Optional - Specific type or category of the error

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
}
