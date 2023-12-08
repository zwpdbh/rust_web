use serde::Deserialize;
use serde::Serialize;

use warp::reject::Reject;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

// create an empty struct for our error type
#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
