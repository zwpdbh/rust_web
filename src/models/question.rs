use serde::Serialize;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

use warp::reject::Reject;

#[derive(Debug, Serialize)]
pub struct QuestionId(pub String);

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(unused)]
pub struct Question {
    pub id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[allow(unused)]
impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

// create an empty struct for our error type
#[derive(Debug)]
pub struct InvalidId;
impl Reject for InvalidId {}
