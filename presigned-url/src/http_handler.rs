use lambda_http::{Body, Error, Request, RequestExt, Response};
use lambda_http::tracing;
use crate::presigned_generator::generate_upload_url;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
    pub bucket_name: String,
    pub filename: String,
    pub description: String,
    pub content_type: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    pub upload_url: String,
    pub file_key: String,
}

const UPLOAD_EXPIRY_SECONDS: u64 = 3600; // 1 hour

const ALLOWED_ORIGIN: &str = "https://aws-lambda-ht.vercel.app";
// For testing
// const ALLOWED_ORIGIN: &str = "http://localhost:3000";

pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    tracing::info!("Event: {:?}", event);
    
    // Get Lambda context via RequestExt
    let ctx = event
        .lambda_context_ref()
        .expect("lambda_context should be available");

    let event_id = &ctx.request_id;

    let bucket_name = match env::var("BUCKET_NAME") {
        Ok(val) if !val.is_empty() => val,
        _ => {
            let resp = Response::builder()
                .status(500)
                .header("Access-Control-Allow-Origin", ALLOWED_ORIGIN)
                .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
                .header("Access-Control-Allow-Headers", "Content-Type, X-Amz-Date, Authorization, X-Api-Key, X-Amz-Security-Token")
                .body("ERROR: BUCKET_NAME environment variable is not set".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };
        
    let filename = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("filename"))
        .expect("filename parameter required");

    let description = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("description"))
        .unwrap_or("Null");

    let content_type = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("content_type"))
        .unwrap_or("application/octet-stream");

    let parameters = Params {
        bucket_name: bucket_name.clone(),
        filename: filename.to_string(),
        description: description.to_string(),
        content_type: content_type.to_string(),
        expires_in: UPLOAD_EXPIRY_SECONDS,
    };

    let presigned_url = generate_upload_url(&parameters).await?;
    tracing::info!("Presigned URL: {}", presigned_url.upload_url);

    let response_json = json!({
        "event_id": event_id,
        "upload_url": presigned_url.upload_url,
        "file_key": presigned_url.file_key,
        "bucket_name": bucket_name,
    }).to_string();

    let resp = Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", ALLOWED_ORIGIN)
        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, X-Amz-Date, Authorization, X-Api-Key, X-Amz-Security-Token")
        .header("Content-Type", "application/json")
        .body(response_json.into())
        .map_err(Box::new)?;
    Ok(resp)
}