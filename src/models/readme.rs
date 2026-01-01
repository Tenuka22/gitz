use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReadmeAnalysis {
    pub questions: Vec<Question>,
    pub extracted: ExtractedData,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub question: String,
    pub options: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExtractedData {
    pub project_name: Option<String>,
    pub project_type: Option<String>,
    pub tech_stack: Vec<String>,
    pub main_functionality: Vec<String>,
    pub inferred_features: Option<Vec<String>>,
}
