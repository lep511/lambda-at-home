use crate::openai::chat::ChatOpenAI;
use crate::openai::libs::ChatResponse;
use lambda_runtime::tracing;
use base64::{Engine as _, engine::general_purpose};
use env_logger::Env;

pub async fn call_llm(
    prompt: &str, 
    image_data: Vec<u8>, 
    file_format: &str
) -> Result<String, Box<dyn std::error::Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let llm = ChatOpenAI::new("gpt-4o");

    // Create the data URL with proper MIME type
    let mime_type = match file_format.to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => return Err(format!("Unsupported image format: {}", file_format).into()),
    };
    
    let image_base64: String = general_purpose::STANDARD.encode(&image_data);

    let response: ChatResponse = llm
        .with_image_data(image_base64, mime_type)
        .invoke(prompt)
        .await?;

    let mut result = String::new();

    match response.choices {
        Some(candidates) => {
            candidates.iter()
                .filter_map(|candidate| candidate
                    .message.as_ref()?
                    .content.as_ref()
                ).for_each(|content| {
                    result.push_str(content);
                });
        }
        None => {
            tracing::error!("No response choices available");
            result.push_str("No response content available.");
        }
    };
    
    Ok(result)
}