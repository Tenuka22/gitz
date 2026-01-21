mod cerebras_provider;
mod gemini_provider;

use crate::models::{cli::CliModel, error::APIError};

pub use cerebras_provider::CerebrasProvider;
pub use gemini_provider::GeminiProvider;

#[async_trait::async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate_content(
        &self,
        system_prompt: Option<&str>,
        user_messages: Vec<&str>,
    ) -> Result<String, APIError>;
}

pub fn create_provider(
    provider: crate::models::cli::Provider,
    model: Option<CliModel>,
) -> Result<Box<dyn AIProvider>, APIError> {
    match provider {
        crate::models::cli::Provider::Gemini => {
            let api_key = std::env::var("GEMINI_API_KEY")
                .map_err(|e| APIError::new("GEMINI_API_KEY not found", e))?;
            Ok(Box::new(GeminiProvider::new(api_key, model)?))
        }
        crate::models::cli::Provider::Cerebras => {
            let api_key = std::env::var("CEREBRAS_API_KEY")
                .map_err(|e| APIError::new("CEREBRAS_API_KEY not found", e))?;
            Ok(Box::new(CerebrasProvider::new(api_key, model)?))
        }
    }
}
