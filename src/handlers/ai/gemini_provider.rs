use super::AIProvider;
use crate::models::{cli::CliModel, error::APIError};
use gemini_rust::{Gemini, Model};

pub struct GeminiProvider {
    client: Gemini,
}

impl GeminiProvider {
    pub fn new(api_key: String, model: Option<CliModel>) -> Result<Self, APIError> {
        let gemini_model = match model {
            None | Some(CliModel::Gemini25Flash) => Model::Gemini25Flash,
            Some(CliModel::Gemini25Pro) => Model::Gemini25Pro,
            Some(CliModel::Gemini25FlashLite) => Model::Gemini25FlashLite,

            Some(other) => {
                return Err(APIError::new_msg(
                    "Gemini",
                    &format!("Model {:?} is not supported by Gemini", other),
                ));
            }
        };

        let client =
            Gemini::with_model(&api_key, gemini_model).map_err(|e| APIError::new("Gemini", e))?;

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
