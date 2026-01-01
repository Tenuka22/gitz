use crate::models::error::APIError;
use super::AIProvider;
use gemini_rust::{Gemini, Model};

pub struct GeminiProvider {
    client: Gemini,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Result<Self, APIError> {
        let client = Gemini::with_model(&api_key, Model::Gemini25Flash)
            .map_err(|e| APIError::new("Gemini", e))?;
        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl AIProvider for GeminiProvider {
    async fn generate_content(
        &self,
        system_prompt: Option<&str>,
        user_messages: Vec<&str>,
    ) -> Result<String, APIError> {
        let mut request = self.client.generate_content();

        if let Some(prompt) = system_prompt {
            request = request.with_system_prompt(prompt);
        }

        for message in user_messages {
            request = request.with_user_message(message);
        }

        let response = request
            .execute()
            .await
            .map_err(|e| APIError::new("Gemini", e))?;

        Ok(response.text())
    }
}
