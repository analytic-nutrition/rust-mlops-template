/* Manta Ray LLM library for climate care initiatives */

use rust_bert::pipelines::question_answering::{QaInput, QuestionAnsweringModel};
use evalexpr::*;

// Function to answer questions about climate care
pub fn answer_climate_question(question: &str, context: &str) -> Vec<String> {
    let qa_model = QuestionAnsweringModel::new(Default::default()).unwrap();
    let qa_input = QaInput {
        question: question.to_string(),
        context: context.to_string(),
    };
    let output = qa_model.predict(&[qa_input], 1, 32);
    output[0].iter().map(|x| x.answer.clone()).collect()
}

// Function to calculate carbon footprint or other metrics
pub fn calculate(expression: &str) -> Result<f64, EvalexprError> {
    eval(expression)
}

// Function to generate a petition text
pub fn generate_petition(corporation: &str, issue: &str) -> String {
    format!(
        "Petition to {}: We demand that {} immediately cease {} to reduce their carbon footprint and contribute to climate care initiatives.",
        corporation, corporation, issue
    )
}