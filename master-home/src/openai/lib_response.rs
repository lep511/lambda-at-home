use serde::{Deserialize, Serialize};
use std::collections::HashMap; 
use serde_json::Value;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ Response ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// check: https://platform.openai.com/docs/api-reference/responses/create

/// OpenAI's most advanced interface for generating model responses. Supports text and image 
/// inputs, and text outputs. Create stateful interactions with the model, using the output 
/// of previous responses as input. Extend the model's capabilities with built-in tools for 
/// file search, web search, computer use, and more. Allow the model access to external 
/// systems and data using function calling.
/// 
/// [Docs OpenAI - Responses](https://platform.openai.com/docs/api-reference/responses)
///
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRequest {
    /// Text, image, or file inputs to the model, used to generate a response.
    pub input: InputContent,

    /// Model ID used to generate the response, like gpt-4o or o1. 
    pub model: String,

    /// **Optional.** Specify additional output data to include in the model response. 
    /// 
    /// # Currently supported values:
    /// * `file_search_call.results`: Include the search results of the file search tool call.
    /// * `message.input_image.image_url`: Include image urls from the input message.
    /// * `computer_call_output.output.image_url`: Include image urls from the computer call output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,

    /// **Optional.** Inserts a system (or developer) message as the first item in the model's 
    /// context. When using along with previous_response_id, the instructions from a previous 
    /// response will be not be carried over to the next response. 
    /// This makes it simple to swap out system (or developer) messages in new responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// **Optional.** An upper bound for the number of tokens that can be generated 
    /// for a response, including visible output tokens 
    /// and [reasoning tokens](https://platform.openai.com/docs/guides/reasoning).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u32>,

    /// **Optional.** Set of 16 key-value pairs that can be attached to an object. 
    /// This can be useful for storing additional information about 
    /// the object in a structured format, and querying 
    /// for objects via API or the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,

    /// **Optional.** Whether to allow the model to run tool calls in parallel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// **Optional.** The unique ID of the previous response to the model. 
    /// Use this to create multi-turn conversations.
    /// Learn more about [conversation state.](https://platform.openai.com/docs/guides/conversation-state)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// **Optional.** Configuration options for reasoning models.
    /// O-series models only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning:  Option<Reasoning>,

    /// **Optional.** Whether to store the generated model response 
    /// for later retrieval via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store:  Option<bool>,

    /// **Optional.** If set to true, the model response data will be streamed to the client 
    /// as it is generated using server-sent events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// **Optional.** What sampling temperature to use, between 0 and 2. Higher values 
    /// like 0.8 will make the output more random, while lower values like 0.2 will 
    /// make it more focused and deterministic. OpenAI recommend altering this 
    /// or top_p but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// **Optional.** Configuration options for a text response from the model. 
    /// Can be plain text or structured JSON data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseFormat>,

    /// **Optional.** How the model should select which tool (or tools) to use when 
    /// generating a response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// **Optional.** An array of tools the model may call while generating a response.
    /// You can specify which tool to use by setting the tool_choice parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Value>>,

    /// **Optional.** An alternative to sampling with temperature, called nucleus sampling, 
    /// where the model considers the results of the tokens with top_p probability mass. 
    /// So 0.1 means only the tokens comprising the top 10% probability 
    /// mass are considered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// **Optional.** The truncation strategy to use for the model response.
    /// 
    /// # Strategies
    /// - **auto**: If the context of this response and previous ones exceeds 
    ///         the model's context window size, the model will truncate 
    ///         the response to fit the context window by dropping input 
    ///         items in the middle of the conversation.
    /// - **disabled** (default):  If a model response will exceed the context window size 
    ///                        for a model, the request will fail with a 400 error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,

    /// **Optional.** A unique identifier representing your end-user, which can help 
    /// OpenAI to monitor and detect abuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Text, image, or file inputs to the model, used to generate a response.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputContent {
    /// A text input to the model, equivalent to a text input with the user role.
    String(String),

    /// A list of one or many input items to the model, containing different
    /// content types.
    ItemList(Vec<InputItemList>),

    Null(Vec<()>),
}

/// A list of one or many input items to the model, 
/// containing different content types.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputItemList {
    /// A file input to the model.
    InputFile(InputFile),
    
    /// An image input to the model. 
    /// Learn about [image inputs](https://platform.openai.com/docs/guides/vision).
    InputImage(InputImage),

    /// A message input to the model with a role indicating instruction following hierarchy. 
    /// Instructions given with the `developer` or `system` role take precedence over instructions 
    /// given with the `user` role. Messages with the `assistant` role are presumed to have been 
    /// generated by the model in previous interactions.
    InputMessage(InputMessage),

    /// A text input to the model.
    InputText(InputText),

    /// An item representing part of the context for the response to be generated by the model. 
    /// Can contain text, images, and audio inputs, as well as previous assistant responses 
    /// and tool call outputs.
    Item(Item),

    ItemReference(ItemReference),

    /// An empty vector.
    Null(Vec<()>),
}

/// A file input to the model.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputFile {
    /// The type of the input item. Always `input_file`.
    #[serde(rename = "type")]
    pub type_: String,

    /// **Optional.** The content of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,

    /// **Optional.** The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// **Optional.** The name of the file to be sent to the model.
    /// A fully qualified URL or base64 encoded image in a data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

/// A text input to the model.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputText {
    /// The content of the input text.
    pub text: String,

    /// The type of the input text. Always `text`.
    #[serde(rename = "type")]
    pub type_: String,
}
/// An image input to the model. 
/// Learn about [image inputs](https://platform.openai.com/docs/guides/vision).
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputImage {
    /// The detail level of the image to be sent to the model. 
    /// One of `high`, `low`, or `auto`. Defaults to `autov.
    pub detail: String,

    /// The type of the input item. Always `input_image`.
    #[serde(rename = "type")]
    pub type_: String,

    /// **Optional.** The ID of the file to be sent to the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    
    /// **Optional.** The URL of the image to be sent to the model. 
    /// A fully qualified URL or base64 encoded image in a data URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}


/// A message input to the model with a role indicating instruction following hierarchy. 
/// Instructions given with the developer or system role take precedence over 
/// instructions given with the user role.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMessage {
    /// A list of one or many input items to the model, containing 
    /// different content types.
    pub content: InputContent,

    /// The role of the message input. One of user, system, or developer.
    pub role: Role,

    /// **Optional.** The status of item. One of `in_progress`, 
    /// `completed`, or `incomplete`. 
    /// Populated when items are returned via API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// **Optional.** The type of the message input. Always set to `message`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// Controls which (if any) tool is called by the model.
    /// `none` means the model will not call any tool and instead generates a message.
    /// `auto` means the model can pick between generating a message or calling one or more tools.
    /// `required` means the model must call one or more tools.
    ToolChoiceMode(String),

    /// Indicates that the model should use a built-in tool to generate a response.
    HostedTool(HostedTool),

    /// Use this option to force the model to call a specific function.
    FunctionTool(FunctionTool),
}

/// Use to force the model to call a specific function.
/// #[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTool {
    /// The name of the function to call.
    pub name: String,

    /// For function calling, the type is always function.
    #[serde(rename = "type")]    
    pub type_: String,
}
/// Indicates that the model should use a built-in tool to generate a response.
/// #[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedTool {
    /// The type of hosted tool the model should to use. Learn more about built-in tools.
    /// # Allowed values are:
    /// * `file_search`
    /// * `web_search_preview`
    /// * `computer_use_preview`
    #[serde(rename = "type")]
    pub type_: String,
}

/// Configuration options for a text response from the model. 
/// Can be plain text or structured JSON data.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {

    pub format: Option<FormatResponse>,
}

/// An object specifying the format that the model must output.
/// - Configuring `{ "type": "json_schema" }`` enables Structured Outputs, 
/// which ensures the model will match your supplied JSON schema.
/// - The default format is `{ "type": "text" }`` with no additional options.
/// 
/// # Not recommended for gpt-4o and newer models:
/// - Setting to `{ "type": "json_object" }`` enables the older JSON mode, 
/// which ensures the message the model generates is valid JSON. 
/// Using json_schema is preferred for models that support it.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormatResponse {
    /// The type of the format. Always `text`.
    TextFormat(TextFormat),

    /// JSON Schema response format. Used to generate structured JSON responses.
    JsonSchema(JsonSchema),

    /// JSON object response format. An older method of generating JSON responses.
    JsonObject(JsonObject),
}

/// Default response format. Used to generate text responses.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextFormat {
    /// The type of the format. Always `text`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// JSON object response format. An older method of generating JSON responses. 
/// Using `json_schema` is recommended for models that support it. Note that the 
/// model will not generate JSON without a system or user message 
/// instructing it to do so.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonObject {
    /// The type of the format. Always `json_object`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// JSON Schema response format. Used to generate structured JSON responses.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    /// The name of the response format. Must be a-z, A-Z, 0-9, 
    /// or contain underscores and dashes, with a maximum length of 64.
    pub name: String,

    /// The schema for the response format, described as a JSON Schema object.
    pub schema: Value,

    /// The type of the format. Always `json_schema`.
    #[serde(rename = "type")]
    pub type_: String,

    /// **Optional.** A description of the response format.
    pub description: Option<String>,

    /// **Optional.** Whether to enable strict schema adherence when generating 
    /// the output. If set to true, the model will always follow the exact schema 
    /// defined in the `schema` field. Only a subset of JSON Schema is supported 
    /// when `strict` is `true`.
    pub strict: Option<bool>,
}

/// An item representing part of the context for the response to be generated 
/// by the model. Can contain text, images, and audio inputs, as well as previous 
/// assistant responses and tool call outputs.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
    /// A message input to the model with a role indicating instruction following hierarchy. 
    /// Instructions given with the developer or system role take precedence over 
    /// instructions given with the user role.
    InputMessage(InputMessage),

    /// An output message from the model.
    OutputMessage(OutputMessage),

    /// The results of a file search tool call. See the file search guide for more information.
    FileSearchTool(FileSearchTool),

    /// A tool call to a computer use tool. See the computer use guide for more information.
    ComputerTool(Value),

    /// The output of a computer tool call.
    ComputerToolOutput(Value),

    /// The results of a web search tool call. See the web search guide for more information.
    WebSearchTool(Value),

    /// A tool call to run a function. See the function calling guide for more information.
    FunctionTool(Value),

    /// The output of a function tool call.
    FunctionToolOutput(Value),

    /// A description of the chain of thought used by a reasoning model while 
    /// generating a response.
    Reasoning(ReasoningItem),
}

/// The results of a file search tool call.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchTool {
    /// The unique ID of the file search tool call.
    pub id: String,

    /// The queries used to search for files.
    pub queries: Vec<String>,

    /// The status of the file search tool call. One of 
    /// in_progress, searching, incomplete or failed,
    pub status: String,

    /// The type of item to reference. Always item_reference.
    #[serde(rename = "type")]
    pub type_: String,

    /// **Optional.** The results of the file search tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<FileSearchToolResult>>,
}

/// The results of the file search tool call.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSearchToolResult {
    /// **Optional.** Set of 16 key-value pairs that can be attached to an object. 
    /// This can be useful for storing additional information about the object in 
    /// a structured format, and querying for objects via API or the dashboard.
    /// Keys are strings with a maximum length of 64 characters. 
    /// Values are strings with a maximum length of 512 characters, booleans, or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Value>,

    /// **Optional.** The unique ID of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// **Optional.** The name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// **Optional.** The relevance score of the file - a value between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,

    /// **Optional.** The text that was retrieved from the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<u32>,
}

/// An internal identifier for an item to reference.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReference {
    /// The ID of the item to reference.
    pub id: String,

    /// The type of item to reference. Always item_reference.
    #[serde(rename = "type")]
    pub type_: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub format: Value,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningItem {
    /// The unique identifier of the reasoning content.
    pub id: String,
    
    /// Reasoning text contents.
    pub summary: Vec<ReasoningContent>,
    
    /// The type of the object. Always `reasoning`.
    #[serde(rename = "type")]
    pub type_: String,

    /// **Optional.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Reasoning text contents.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningContent {
    /// A short summary of the reasoning used by the model 
    /// when generating the response.
    pub text: String,

    /// The type of the object. Always `summary_text`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Configuration options for reasoning models.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reasoning {
    /// **Optional.** Constrains effort on reasoning for reasoning models. 
    /// Currently supported values are `low`, `medium`, and `high`. 
    /// Reducing reasoning effort can result in faster responses and fewer 
    /// tokens used on reasoning in a response.
    /// O-series models only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effort: Option<String>,

    /// **Optional.** A summary of the reasoning performed by the model. 
    /// This can be useful for debugging and understanding the model's reasoning process. 
    /// One of `concise` or `detailed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_summary: Option<String>,
}

/// An output message from the model.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputMessage {
    /// The content of the output message.
    pub content: Vec<OutputContent>,

    /// The unique ID of the output message.
    pub id: String,

    /// The role of the output message. Always `assistant`.
    pub role: Role,

    /// The status of the message input. One of `in_progress`, `completed`, or 
    /// `incomplete`. Populated when input items are returned via API.
    pub status: String,

    /// The type of the output message. Always `message`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// The content of the output message.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutputContent {
    /// A text output from the model.
    OutputText(OutputText),

    /// A refusal from the model.
    Refusal(Refusal),

    /// An empty vector.
    Null(Vec<()>),
}

/// A refusal from the model.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Refusal {
    /// The refusal explanationfrom the model.
    pub refusal: String,

    /// The type of the refusal. Always `refusal`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// A text output from the model.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputText {

    /// The annotations of the text output.
    pub annotations: Vec<Annotation>,

    /// The text output from the model.
    pub text: String,

    /// The type of the output text. Always `output_text`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// The annotations of the text output.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Annotation {
    /// A citation to a file.
    FileCitation(FileCitation),
    
    /// A citation for a web resource used to generate 
    /// a model response.
    UrlCitation(UrlCitation),

    /// A path to a file.
    FilePath(FilePath),
}

/// A citation to a file.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCitation {
    /// The ID of the file.
    pub file_id: String,

    /// The index of the file in the list of files.
    pub index: u32,

    /// The type of the citation. Always `file_citation`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// A citation for a web resource used to generate 
/// a model response.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UrlCitation {
    /// The index of the last character of the URL citation in the message.
    pub end_index: u32,

    /// The index of the first character of the URL citation in the message.
    pub start_index: u32,
    
    /// The title of the web resource.
    pub title: String,

    /// The URL of the web resource.
    pub url: String,

    /// The type of the citation. Always `url_citation`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// A path to a file.
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilePath {
    /// The ID of the file.
    pub file_id: String,
    
    /// The index of the file in the list of files.
    pub index: u32,

    /// The type of the citation. Always `file_path`.
    #[serde(rename = "type")]
    pub type_: String,
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
pub struct Source {
    #[serde(rename = "type")]
    pub type_: String,

    pub media_type: String,

    pub data: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ResponseObject {
    /// Unix timestamp (in seconds) of when this Response was created.
    pub created_at: i64,

    /// An error object returned when the model fails to generate a Response.
    pub error: Option<Value>,

    /// nique identifier for this Response.
    pub id: Option<String>,

    /// Details about why the response is incomplete.
    pub incomplete_details: Option<Value>,

    /// Inserts a system (or developer) message as the first item in the model's context.
    /// When using along with previous_response_id, the instructions from a previous 
    /// response will not be carried over to the next response. 
    /// This makes it simple to swap out system (or developer) messages in new responses.
    pub instructions: Option<Value>,

    /// An upper bound for the number of tokens that can be generated for a response, 
    /// including visible output tokens and reasoning tokens.
    pub max_output_tokens: Option<u64>,

    /// Set of 16 key-value pairs that can be attached to an object. 
    /// This can be useful for storing additional information about the object 
    /// in a structured format, and querying for objects via API or the dashboard.
    ///
    /// Keys are strings with a maximum length of 64 characters. 
    /// Values are strings with a maximum length of 512 characters.
    pub metadata: Option<HashMap<String, Value>>,

    /// Model ID used to generate the response, like gpt-4o or o1. 
    pub model: Option<String>,

    /// The object type of this resource - always set to `response`.
    pub object: Option<String>,

    /// An array of content items generated by the model.
    /// 
    /// * The length and order of items in the output array is dependent 
    /// on the model's response.
    /// * Rather than accessing the first item in the output array and 
    /// assuming it's an assistant message with the content generated by 
    /// the model, you might consider using the output_text 
    /// property where supported in SDKs.
    pub output: Vec<OutputItem>,

    /// SDK-only convenience property that contains the aggregated text 
    /// output from all output_text items in the output array, if any are present. 
    /// Supported in the Python and JavaScript SDKs.
    pub output_text: Option<String>,

    /// Whether to allow the model to run tool calls in parallel.
    pub parallel_tool_calls: Option<bool>,

    /// The unique ID of the previous response to the model. Use this to 
    /// create multi-turn conversations. Learn more about conversation state.
    pub previous_response_id: Option<String>,

    /// A description of the chain of thought used by a reasoning model while 
    /// generating a response.
    pub reasoning: Option<ReasoningResponse>,

    /// The status of the response generation. One of `completed`, `failed`, 
    /// `in_progress`, or `incomplete`.
    pub status: Option<String>,

    pub store: Option<bool>,

    /// What sampling temperature to use, between 0 and 2.
    pub temperature: Option<f64>,

    /// Configuration options for a text response from the model.
    pub text: Option<TextInfo>,

    /// How the model should select which tool (or tools) to use when generating a response.
    pub tool_choice: Option<String>,

    /// An array of tools the model may call while generating a response. 
    /// You can specify which tool to use by setting the tool_choice parameter.
    pub tools: Option<Vec<Value>>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model 
    /// considers the results of the tokens with top_p probability mass. 
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    pub top_p: Option<f64>,

    /// The truncation strategy to use for the model response.
    /// * `auto`
    /// * `disabled`
    pub truncation: Option<String>,

    /// Represents token usage details including input tokens, output tokens, 
    /// a breakdown of output tokens, and the total tokens used.
    pub usage: Option<Usage>,

    /// A unique identifier representing your end-user, which can help OpenAI 
    /// to monitor and detect abuse.
    pub user: Option<String>,
}

/// Different kinds of items in the main "output" array
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")] // Use the 'type' field in JSON to decide which variant
#[serde(rename_all = "snake_case")] // Maps JSON "reasoning" to Rust Reasoning variant etc.
pub enum OutputItem {
    Reasoning(ReasoningOutput),
    Message(MessageOutput),
    WebSearchCall(Value),
}

// Specific struct for the "reasoning" type output
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ReasoningOutput {
    pub id: String,
    pub summary: Vec<Value>, // Using Value for flexibility
    // The 'type' field is handled by the enum tag
}

// Specific struct for the "message" type output
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct MessageOutput {
    pub id: String,
    pub content: Vec<ContentItem>,
    pub role: String,
    pub status: String,
    // The 'type' field is handled by the enum tag
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ContentItem {
    pub annotations: Vec<Value>,
    pub text: String,
    #[serde(rename = "type")]
    pub content_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct TextInfo {
    pub format: TextFormat,
}

/// Represents token usage details including input tokens, output tokens, 
/// a breakdown of output tokens, and the total tokens used.
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct Usage {
    /// The number of input tokens.
    pub input_tokens: Option<u64>,

    /// A detailed breakdown of the input tokens.
    pub input_tokens_details: Option<InputTokensDetails>,

    /// The number of output tokens.
    pub output_tokens: Option<u64>,

    /// A detailed breakdown of the output tokens.
    pub output_tokens_details: Option<OutputTokensDetails>,

    /// The total number of tokens used.
    pub total_tokens: Option<u64>,
}

/// A detailed breakdown of the input tokens.
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct InputTokensDetails {
    /// The number of tokens that were retrieved from the cache.
    pub cached_tokens: u64,
}

/// A detailed breakdown of the output tokens.
#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct OutputTokensDetails {
    /// The number of reasoning tokens.
    pub reasoning_tokens: u64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
pub struct ReasoningResponse {
    pub effort: Option<String>,
    pub generate_summary: Option<Value>,
}