use crate::models::error::APIError;
use super::AIProvider;
use serde::{Deserialize, Serialize};

const CEREBRAS_API_URL: &str = "https://api.cerebras.ai/v1/chat/completions";
const DEFAULT_MODEL: &str = "gpt-oss-120b";

pub struct CerebrasProvider {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

impl CerebrasProvider {
    pub fn new(api_key: String) -> Result<Self, APIError> {
        let client = reqwest::Client::new();
        Ok(Self { api_key, client })
    }
}

#[async_trait::async_trait]
impl AIProvider for CerebrasProvider {
    async fn generate_content(
        &self,
        system_prompt: Option<&str>,
        user_messages: Vec<&str>,
    ) -> Result<String, APIError> {
        let mut messages = Vec::new();

        // Add system message if provided
        if let Some(prompt) = system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: prompt.to_string(),
            });
        }

        // Add user messages
        let combined_message = user_messages.join("\n\n");
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: combined_message,
        });

        let request_body = ChatCompletionRequest {
            model: DEFAULT_MODEL.to_string(),
            messages,
        };

        let response = self.client
            .post(CEREBRAS_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| APIError::new_msg("Cerebras", &format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(APIError::new_msg(
                "Cerebras",
                &format!("API error ({}): {}", status, error_text),
            ));
        }

        let completion: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| APIError::new_msg("Cerebras", &format!("Failed to parse response: {}", e)))?;

        completion
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| APIError::new_msg("Cerebras", "No choices in response"))
    }
}
