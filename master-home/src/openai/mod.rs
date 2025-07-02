use std::time::Duration;

pub mod chat;
pub mod response;
pub mod embed;
pub mod error;
pub mod libs;
pub mod lib_response;
pub mod utils;
pub mod requests;

pub const RETRY_BASE_DELAY: Duration = Duration::from_secs(2);

pub static OPENAI_BASE_URL: &str = "https://api.openai.com/v1/chat/completions";
pub static OPENAI_RESPONSE_URL: &str = "https://api.openai.com/v1/responses";
pub static OPENAI_EMBED_URL: &str = "https://api.openai.com/v1/embeddings";
