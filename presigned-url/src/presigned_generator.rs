// Generate pre-signed URL for UPLOADING files
use aws_sdk_s3::{Client, presigning::PresigningConfig};
use crate::http_handler::{UploadResponse, Params};
use uuid::Uuid;

pub async fn generate_upload_url(
    params: &Params,
) -> Result<UploadResponse, aws_sdk_s3::Error> {
    let config = aws_config::load_from_env().await;
    let s3_client = Client::new(&config);

    // Generate unique key for the file
    let file_extension = params.filename
        .split('.')
        .last()
        .unwrap_or("bin");

    let file_key = format!("uploads/{}.{}", 
        Uuid::new_v4(),
        file_extension
    );
        
    // Set bucket bucket_name
    let bucket_name = &params.bucket_name;

    // Set the content-type
    let content_type = &params.content_type;

    // Set the expiry for the pre-signed URL
    let upload_expiry_time = params.expires_in;
    
    // Pre-signing configuration
    let presigning_config = PresigningConfig::builder()
        .expires_in(std::time::Duration::from_secs(upload_expiry_time))
        .build()
        .expect("less than one week");
    
    // Generate pre-signed URL for PUT
    let presigned_request = s3_client
        .put_object()
        .bucket(bucket_name)
        .key(&file_key)
        .content_type(content_type)
        .metadata("file-name", params.filename.as_str())
        .metadata("description", params.description.as_str())
        .presigned(presigning_config)
        .await?;
    
    Ok(UploadResponse {
        upload_url: presigned_request.uri().to_string(),
        file_key: file_key,
    })
}
