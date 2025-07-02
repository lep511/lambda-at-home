use futures::pin_mut;
use futures::StreamExt;
use async_stream::stream;
use crate::openai::requests::{request_chat, strem_chat};
use crate::openai::utils::GetApiKey;
use crate::openai::libs::{
    MainRequest, ChatRequest, InputContent, ResponseFormat,
    Message, Role, ChatResponse, ImageUrl,
};
use crate::openai::OPENAI_BASE_URL;
use crate::openai::error::OpenAIError;
use std::time::Duration;
use log::error;


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChatOpenAI {
    pub api_key: String,
    pub request: ChatRequest,
    pub timeout: Duration,
    pub max_retries: u32,
}

#[allow(dead_code)]
impl ChatOpenAI {
    pub fn new(model: &str) -> Self {
        let api_key: String = match Self::get_api_key() {
            Ok(api_key) => api_key,
            Err(_) => "not_key".to_string()
        };

        let request = ChatRequest {
            model: model.to_string(),
            messages: None,
            temperature: None,
            tools: None,
            tool_choice: None,
            max_completion_tokens: None,
            response_format: None,
            frequency_penalty: None,
            presence_penalty: None,
            top_p: None,
            stream: Some(false),
            n_completion: Some(1),
            stop: None,
        };
        
        Self {
            api_key: api_key,
            request: request,
            timeout: Duration::from_secs(300), // default: 5 minutes
            max_retries: 3,         // default: 3 times
        }
    }

    pub async fn invoke(
        mut self,
        prompt: &str,
    ) -> Result<ChatResponse, OpenAIError> {
        
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(prompt.to_string()),
            source: None,
            image_url: None,
        }];

        let new_message = Message {
            role: Role::User,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        let body_request = MainRequest::Chat(self.request.clone());

        let response: String = match request_chat(
            &body_request,
            OPENAI_BASE_URL,
            &self.api_key,
            self.timeout,
            self.max_retries,
        ).await {
            Ok(response) => response,
            Err(openai_error) => {
                return Err(openai_error);
            }
        };
        
        let chat_response: ChatResponse = match serde_json::from_str(&response) {
            Ok(response_form) => response_form,
            Err(e) => {
                error!("Failed to parse response: {}. ERROR-req-0023", e);
                return Err(OpenAIError::ResponseContentError);
            }
        };

        let format_response = ChatResponse {
            choices: chat_response.choices,
            created: chat_response.created,
            id: chat_response.id,
            model: chat_response.model,
            object: chat_response.object,
            system_fingerprint: chat_response.system_fingerprint,
            usage: chat_response.usage,
            chat_history: self.request.messages,
            service_tier: None,
            error: None,
        };
        Ok(format_response)
    }

    pub fn stream_response(
        mut self,
        prompt: String,  // Don't change type for stream
    ) -> impl futures::Stream<Item = ChatResponse> {
        stream! {     
            
            let content = vec![InputContent {
                content_type: "text".to_string(),
                text: Some(prompt),
                source: None,
                image_url: None,
            }];       
            
            let new_message = Message {
                role: Role::User,
                content: content.clone(),
                recipient: None,
                end_turn: None,
            };
    
            if let Some(messages) = &mut self.request.messages {
                messages.push(new_message);
            } else {
                self.request.messages = Some(vec![new_message]);
            }

            self.request.stream = Some(true);
            let endpoint_string = OPENAI_BASE_URL.to_string();

            let stream = strem_chat(
                endpoint_string.clone(),
                self.api_key.clone(),
                self.request.clone(),
            );

            pin_mut!(stream);

            while let Some(chat_response) = stream.next().await {
                yield chat_response;
            }
        }
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
        self.request.max_completion_tokens = Some(max_tokens);
        self
    }

    pub fn with_timeout_sec(mut self, timeout: u64) -> Self {
        self.timeout = Duration::from_secs(timeout);
        self
    }

    pub fn with_frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        if frequency_penalty < -2.0 || frequency_penalty > 2.0 {
            println!(
                "[ERROR] Frequency penalty must be between -2.0 and 2.0. Actual frequency penalty is {}",
                self.request.frequency_penalty.unwrap_or(0.0)
            );
            self
        } else {
            self.request.frequency_penalty = Some(frequency_penalty);
            self
        }
    }

    pub fn with_presence_penalty(mut self, presence_penalty: f32) -> Self {
        if presence_penalty < -2.0 || presence_penalty > 2.0 {
            println!(
                "[ERROR] Presence penalty must be between -2.0 and 2.0. Actual presence penalty is {}",
                self.request.presence_penalty.unwrap_or(0.0)
            );
            self
        } else {
            self.request.presence_penalty = Some(presence_penalty);
            self
        }
    }

    pub fn with_image_data(mut self, image_base64: String, mime_type: &str) -> Self {
        // Mime type supported: [image/png, image/jpeg, image/gif, image/webp] 
        let image_url = format!("data:{};base64,{}", mime_type, image_base64);

        let url = ImageUrl {
            url: image_url,
        };

        let content = vec![InputContent {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(url),
        }];

        let new_message = Message {
            role: Role::User,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }
        self
    }

    pub fn with_image_url(mut self, image_url: &str) -> Self {
        let url = ImageUrl {
            url: image_url.to_string(),
        };
        
        let content = vec![InputContent {
            content_type: "image_url".to_string(),
            text: None,
            source: None,
            image_url: Some(url),
        }];

        let new_message = Message {
            role: Role::User,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }
        self
    }

    pub fn with_top_p(mut self, top_p: f32) -> Self {
        if top_p < 0.0 || top_p > 1.0 {
            println!(
                "[ERROR] Top p must be between 0.0 and 1.0. Actual top p is {}",
                self.request.top_p.unwrap_or(0.0)
            );
            self
        } else {
            self.request.top_p = Some(top_p);
            self
        }
    }

    pub fn with_n_completion(mut self, n_completion: u32) -> Self {
        self.request.n_completion = Some(n_completion);
        self
    }

    pub fn with_stop(mut self, stop: Vec<String>) -> Self {
        self.request.stop = Some(stop);
        self
    }

    pub fn with_system_prompt(mut self, system_prompt: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(system_prompt.to_string()),
            source: None,
            image_url: None,
        }];

        let new_message = Message {
            role: match self.request.model.as_str() {
                "o1-mini" => Role::User,
                _ => Role::Developer,
            },
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.insert(0, new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_assistant_response(mut self,  assistant_response: &str) -> Self {
        let content = vec![InputContent {
            content_type: "text".to_string(),
            text: Some(assistant_response.to_string()),
            source: None,
            image_url: None,
        }];

        let new_message = Message {
            role: Role::Assistant,
            content: content.clone(),
            recipient: None,
            end_turn: None,
        };

        if let Some(messages) = &mut self.request.messages {
            messages.push(new_message);
        } else {
            self.request.messages = Some(vec![new_message]);
        }

        self
    }

    pub fn with_chat_history(mut self, history: Vec<Message>) -> Self {
        self.request.messages = Some(history);
        self
    }

    pub fn with_json_schema(mut self, json_schema: serde_json::Value) -> Self {
        let response_format = ResponseFormat {
            response_type: "json_schema".to_string(),
            json_schema: Some(json_schema),
        };

        self.request.response_format = Some(response_format);
        self
    }

    pub fn with_tools(mut self, tools_data: Vec<serde_json::Value>) -> Self {
        self.request.tools = Some(tools_data);
        self
    }

    pub fn with_tool_choice(mut self, tool_choice: serde_json::Value) -> Self {
        self.request.tool_choice = Some(tool_choice);
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }
}

impl GetApiKey for ChatOpenAI {}