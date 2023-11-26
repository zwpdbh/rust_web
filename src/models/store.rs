use super::question::Question;
use super::question::QuestionId;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use warp::reject::Reject;

#[derive(Debug, Clone, Deserialize)]
pub struct Store {
    pub questions: HashMap<QuestionId, Question>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("questions.json");
        serde_json::from_str(file).expect("can't read questions.json in the project root")
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}
impl Reject for Error {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start_num = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;

        let end_num = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        return Ok(Pagination {
            start: start_num,
            end: end_num,
        });
    }

    Err(Error::MissingParameters)
}
