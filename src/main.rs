// Import the question module
mod models {
    pub mod question;
}

// Use the Question struct and QuestionId type from the question module
use models::question::{Question, QuestionId};
use std::str::FromStr;

fn main() {
    let question = Question::new(
        // QuestionId("1".to_string()),
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("{:#?}", question);
}
