mod prompts;
mod file_filtering;
mod analysis;
mod generator;

use crate::{
    models::{error::APIError, ui, cli::Provider},
};
use analysis::analyze_readme_content;
use generator::generate_final_readme;


pub async fn handle_readme(provider: Provider) -> Result<(), APIError> {
    ui::Logger::header("README GENERATOR");

    let (analysis, git_context, answers) = analyze_readme_content(provider.clone()).await?;

    generate_final_readme(provider, analysis, git_context, answers).await?;

    Ok(())
}
