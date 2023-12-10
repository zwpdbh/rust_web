use crate::types::answer::{Answer, AnswerId, NewAnswer};
use crate::types::question::{NewQuestion, Question, QuestionId};
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;
// use std::collections::HashMap;
// use std::sync::Arc;
// use tokio::sync::RwLock;
use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    // pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    // pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
    pub connection: PgPool,
}

impl Store {
    // pub fn new() -> Self {
    //     Store {
    //         questions: Arc::new(RwLock::new(Self::init())),
    //         answers: Arc::new(RwLock::new(HashMap::new())),
    //     }
    // }

    // fn init() -> HashMap<QuestionId, Question> {
    //     let file = include_str!("questions.json");
    //     serde_json::from_str(file).expect("can't read questions.json in the project root")
    // }

    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Could not connect to DB via: {}", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, Error> {
        let query_result = sqlx::query("select * from questions limit $1 offset $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await;

        match query_result {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        let query_result = 
            sqlx::query("insert into questions (title, content, tags) values ($1, $2, $3) returning id, title, content, tags")
            .bind(new_question.title)
            .bind(new_question.content)
            .bind(new_question.tags)
            .map(|row: PgRow|{
                Question{
                    id: QuestionId(row.get("id")),
                    title: row.get("title"),
                    content: row.get("content"),
                    tags: row.get("tags")
                }
            }).fetch_one(&self.connection).await;

        match query_result {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn update_question(&self, question: Question, question_id: i32) -> Result<Question, Error> {
        match sqlx::query("update questions set title = $1, content = $2, tags = $3 where id = $4 returning id, title, conent, tags")
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(question_id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags")
        }).fetch_one(&self.connection)
        .await {
            Ok(question) => Ok(question),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }

    pub async fn delete_question(&self, question_id: i32) -> Result<bool, Error> {
        match sqlx::query("delete from questions where id = $1").bind(question_id).execute(&self.connection).await {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            },
        }
    }

    pub async fn add_answer(&self, new_answer: NewAnswer) -> Result<Answer, Error> {
        match sqlx::query("insert into answers (content, question_id) values ($1, $2)")
        .bind(new_answer.content)
        .bind(new_answer.question_id.0)
        .map(|row: PgRow| Answer{
            id: AnswerId(row.get("id")),
            content: row.get("content"),
            question_id: QuestionId(row.get("question_id")),
        })
        .fetch_one(&self.connection)
        .await {
            Ok(answer) => Ok(answer),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError)
            }
        }
    }
}
