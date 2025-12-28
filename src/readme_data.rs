use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ReadmeAnalysis {
    pub questions: Vec<Question>,
    pub extracted: Extracted,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    pub qe: String,
    pub and: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Extracted {
    pub project_name: Option<String>,
    pub project_main_points: Vec<String>,
    pub tech_stack: Vec<String>,
}
