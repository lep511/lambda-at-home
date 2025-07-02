use crate::openai::requests::request_chat;
use crate::openai::utils::GetApiKey;
// use crate::openai::libs::{
//     ChatRequest, ResponseFormat,
//     Message, Role, ChatResponse, ImageUrl,
// };

use crate::openai::libs::MainRequest;
use crate::openai::lib_response::{
    ResponseRequest, InputContent, ResponseObject, ToolChoice,
};
use crate::openai::OPENAI_RESPONSE_URL;
use crate::openai::error::OpenAIError;
use std::time::Duration;
use log::error;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ResponseOpenAI {
    pub api_key: String,
    pub request: ResponseRequest,
    pub timeout: Duration,
    pub max_retries: u32,
}

#[allow(dead_code)]
impl ResponseOpenAI {
    pub fn new(model: &str) -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };

        let request = ResponseRequest {
            input: InputContent::Null(vec![]),
            model: model.to_string(),
            include: None,
            instructions: None,
            max_output_tokens: None,
            metadata: None,
            parallel_tool_calls: None,
            previous_response_id: None,
            reasoning: None,
            store: None,
            stream: None,
            temperature: None,
            text: None,
            tool_choice: None,
            tools: None,
            top_p: None,
            truncation: None,
            user: None,
        };
        
        Self {
            api_key: api_key,
            request: request,
            timeout: Duration::from_secs(300), // default: 5 minutes
            max_retries: 3,         // default: 3 times
        }
    }

    pub async fn invoke(
        self,
    ) -> Result<ResponseObject, OpenAIError> {
        let body_request = MainRequest::Responses(self.request.clone());

        let response: String = match request_chat(
            &body_request,
            OPENAI_RESPONSE_URL,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(openai_error) => {
                return Err(openai_error);
            }
        };
        
        let chat_response: ResponseObject = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Failed to parse response: {}. ERROR-req-0023", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };

        Ok(chat_response)
    }

    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.request.input = InputContent::String(prompt.to_string());
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        if temperature < 0.0 || temperature > 2.0 {
            println!(
                "[ERROR] Temperature must be between 0.0 and 2.0. Actual temperature is {}", 
                self.request.temperature.unwrap_or(0.0)
            );
            self
        } else {
            self.request.temperature = Some(temperature);
            self
        }
    }
    
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.request.max_output_tokens = Some(max_tokens);
        self
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = Duration::from_secs(timeout);
        self
    }

    pub fn with_tools(mut self, tools_data: Vec<serde_json::Value>) -> Self {
        self.request.tools = Some(tools_data);
        self
    }

    pub fn with_tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }

    // pub fn with_tools(mut self, tools_data: Vec<serde_json::Value>) -> Self {
    //     self.request.tools = Some(tools_data);
    //     self
    // }

    // pub fn with_tool_choice(mut self, tool_choice: serde_json::Value) -> Self {
    //     self.request.tool_choice = Some(tool_choice);
    //     self
    // }

    // pub fn with_image_url(mut self, image_url: &str) -> Self {
    //     self.request.input = InputContent::ImageUrl(InputItemList {
    //         image_url: Some(image_url.to_string()),
    //         text: None,
    //     });

    //     self
    // }

    // pub fn with_top_p(mut self, top_p: f32) -> Self {
    //     if top_p < 0.0 || top_p > 1.0 {
    //         println!(
    //             "[ERROR] Top p must be between 0.0 and 1.0. Actual top p is {}",
    //             self.request.top_p.unwrap_or(0.0)
    //         );
    //         self
    //     } else {
    //         self.request.top_p = Some(top_p);
    //         self
    //     }
    // }

    // pub fn with_n_completion(mut self, n_completion: u32) -> Self {
    //     self.request.n_completion = Some(n_completion);
    //     self
    // }

    // pub fn with_stop(mut self, stop: Vec<String>) -> Self {
    //     self.request.stop = Some(stop);
    //     self
    // }

    // pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
    //     let content = vec![InputContent {
    //         content_type: "text".to_string(),
    //         text: Some(system_prompt.to_string()),
    //         source: None,
    //         image_url: None,
    //     }];

    //     let new_message = Message {
    //         role: match self.request.model.as_str() {
    //             "o1-mini" => Role::User,
    //             _ => Role::Developer,
    //         },
    //         content: content.clone(),
    //         recipient: None,
    //         end_turn: None,
    //     };

    //     if let Some(messages) = &mut self.request.messages {
    //         messages.insert(0, new_message);
    //     } else {
    //         self.request.messages = Some(vec![new_message]);
    //     }

    //     self
    // }

    // pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
    //     let content = vec![InputContent {
    //         content_type: "text".to_string(),
    //         text: Some(assistant_response.to_string()),
    //         source: None,
    //         image_url: None,
    //     }];

    //     let new_message = Message {
    //         role: Role::Assistant,
    //         content: content.clone(),
    //         recipient: None,
    //         end_turn: None,
    //     };

    //     if let Some(messages) = &mut self.request.messages {
    //         messages.push(new_message);
    //     } else {
    //         self.request.messages = Some(vec![new_message]);
    //     }

    //     self
    // }

    // pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
    //     self.request.messages = Some(history);
    //     self
    // }

    // pub fn with_json_schema(mut self, json_schema: serde_json::Value) -> Self {
    //     let response_format = ResponseFormat {
    //         response_type: "json_schema".to_string(),
    //         json_schema: Some(json_schema),
    //     };

    //     self.request.response_format = Some(response_format);
    //     self
    // }

    // pub fn with_max_retries(mut self, max_retries: u32) -> Self {
    //     self.max_retries = max_retries;
    //     self
    // }

    // pub fn with_api_key(mut self, api_key: &str) -> Self {
    //     self.api_key = api_key.to_string();
    //     self
    // }

    // pub fn with_json_format(mut self) -> Self {
    //     let text_format = TextFormat {
    //         format: json!({
    //             "type": "json_object"
    //         })
    //     };
    //     self.request.text = Some(text_format);
    //     self
    // }

    // pub fn with_store(mut self, store: bool) -> Self {
    //     self.request.store = Some(store);
    //     self
    // }

    // pub fn with_reasoning(mut self, effort: &str) -> Self {
    //     let reasoning = Reasoning {
    //         effort: effort.to_string(),
    //     };

    //     self.request.reasoning = Some(reasoning);
    //     self
    // }
}

impl GetApiKey for ResponseOpenAI {}