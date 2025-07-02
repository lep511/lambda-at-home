use std::env;
use crate::openai::error::OpenAIError;
use serde_json::{json, Value};
use schemars::schema::RootSchema;
use log::{info, error};

/// Gets the API key from the environment variables
///
/// # Returns
/// * A string slice containing the API key
///
/// # Panics
/// * If the OPENAI_API_KEY environment variable is not set
///
/// # Examples
/// ```
/// let api_key = get_api_key();
/// println!("API key: {}", api_key);
/// ```
pub trait GetApiKey {
    fn get_api_key() -> Result<String, OpenAIError> {
        match env::var("OPENAI_API_KEY") {
            Ok(key) => Ok(key),
            Err(env::VarError::NotPresent) => {
                info!("OPENAI_API_KEY not found in environment variables");
                Err(OpenAIError::ApiKeyNotFound)
            }
            Err(e) => {
                error!("Unable to read env OPENAI_API_KEY {:?}", e);
                Err(OpenAIError::EnvError(e))
            }
        }
    }
}

/// Prints the given request as a pretty-printed JSON string
///
/// # Arguments
/// * `request` - The request to be printed
///
pub fn print_pre(request: &impl serde::Serialize, active: bool) {
    if !active {
        println!();
    } else {
        match serde_json::to_string_pretty(request) {
            Ok(json) => println!("Pretty-printed JSON:\n{}", json),
            Err(e) => error!("Error {:?}", e)
        }
    }
}

/// Transforms a JSON schema into a simplified representation
///
/// # Arguments
///
/// * `schema` - A RootSchema instance containing the JSON schema to transform
/// * `sub_struct` - A boolean flag indicating whether to wrap the schema in an array structure
///
/// # Returns
///
/// Returns a Result containing either:
/// * Ok(Value) - A serde_json::Value containing the transformed schema 
/// * Err(Box<dyn Error>) - An error if:
///   - The schema cannot be serialized to JSON
///   - The serialized JSON is not an object
///
pub fn generate_schema(
    schema: RootSchema,
    name: &str,
    strict: bool,
    additional_properties: bool,
    sub_struct: bool
) -> Result<Value, Box<dyn std::error::Error>> {
    let response_json = match serde_json::to_value(schema) {
        Ok(value) => value,
        Err(e) => return Err(Box::new(e)),
    };

    let mut response = match response_json.as_object() {
        Some(obj) => obj.clone(),
        None => return Err("Serialized JSON is not an object".into()),
    };

    response.remove("$schema");
    response.remove("title");
    response.remove("definitions");

    if sub_struct {
        let mut format_response = json!({
            "type": "array",
            "items": response
        });
        let a_resp = Value::Bool(additional_properties);
        format_response["items"]["additionalProperties"] = a_resp;
        
        return Ok(format_response);
    } else {
        let mut format_response = json!({
            "name": name,
            "schema": response,
            "strict": strict
        });
        let a_resp = Value::Bool(additional_properties);
        format_response["schema"]["additionalProperties"] = a_resp;
        return Ok(format_response);
    }
}