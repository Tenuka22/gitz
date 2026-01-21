mod analysis;
mod file_filtering;
mod generator;
mod prompts;

use crate::models::{
    cli::{CliModel, Provider},
    error::APIError,
    ui,
};
use analysis::analyze_readme_content;
use generator::generate_final_readme;

pub async fn handle_readme(provider: Provider, model: Option<CliModel>) -> Result<(), APIError> {
    ui::Logger::header("README GENERATOR");

    let (analysis, git_context, answers) =
        analyze_readme_content(provider.clone(), model.clone()).await?;

    generate_final_readme(provider, model, analysis, git_context, answers).await?;

    Ok(())
}
