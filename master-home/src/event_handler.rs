use aws_lambda_events::event::eventbridge::EventBridgeEvent;
use aws_sdk_s3::primitives::AggregatedBytes;
use aws_sdk_s3::Client;
use lambda_runtime::{Error, LambdaEvent, tracing};
use crate::llm::call_llm;
use std::path::Path;

fn extract_image_extension(file_path: &str) -> String {   
    let path = Path::new(file_path);
    
    // Extract the extension
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase())
        .unwrap_or_else(|| "unknown".to_string());
    
    return extension;
}

pub(crate)async fn function_handler(event: LambdaEvent<EventBridgeEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let payload = event.payload;
    tracing::info!("Payload: {:?}", payload);
    
    let bucket_name = payload.detail
        .get("bucket")
        .and_then(|b| b.get("name"))
        .and_then(|n| n.as_str())
        .ok_or("The bucket name could not be extracted.")?;
    
    let object_key = payload.detail
        .get("object")
        .and_then(|o| o.get("key"))
        .and_then(|k| k.as_str())
        .ok_or("The object key could not be extracted")?;

    tracing::info!("Bucket name: {}", bucket_name);
    tracing::info!("Object key: {}", object_key);

    // Create an AWS SDK client
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Download the object from S3
    let image_data = client
        .get_object()
        .bucket(bucket_name)
        .key(object_key)
        .send()
        .await?
        .body
        .collect()
        .await?
        .into_bytes()
        .to_vec();

    let file_format = extract_image_extension(object_key);
    tracing::info!("File format: {}", file_format);

    let user_message = "Describe this image";
    let response = match call_llm(user_message, image_data, &file_format).await {
        Ok(response) => response,
        Err(error) => {
            tracing::error!("Error: {}", error);
            error.to_string()
        }
    };
    tracing::info!("Response: {}", response);

    Ok(())
}
