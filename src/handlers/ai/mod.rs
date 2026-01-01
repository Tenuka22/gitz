mod gemini_provider;
mod cerebras_provider;

use crate::models::error::APIError;

pub use gemini_provider::GeminiProvider;
pub use cerebras_provider::CerebrasProvider;

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
) -> Result<Box<dyn AIProvider>, APIError> {
    match provider {
        crate::models::cli::Provider::Gemini => {
            let api_key = std::env::var("GEMINI_API_KEY")
                .map_err(|e| APIError::new("GEMINI_API_KEY not found", e))?;
            Ok(Box::new(GeminiProvider::new(api_key)?))
        }
        crate::models::cli::Provider::Cerebras => {
            let api_key = std::env::var("CEREBRAS_API_KEY")
                .map_err(|e| APIError::new("CEREBRAS_API_KEY not found", e))?;
            Ok(Box::new(CerebrasProvider::new(api_key)?))
        }
    }
}
