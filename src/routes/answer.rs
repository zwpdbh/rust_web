use crate::store;
use crate::types::answer::NewAnswer;
// use crate::types::question::QuestionId;
// use std::collections::HashMap;
use warp::http::StatusCode;

// pub async fn add_answer(
//     store: store::Store,
//     params: HashMap<String, String>,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let answer = Answer {
//         id: AnswerId(1),
//         content: params.get("content").unwrap().to_string(),
//         question_id: QuestionId(params.get("question_id").unwrap().parse().unwrap()),
//     };
//     store
//         .answers
//         .write()
//         .await
//         .insert(answer.id.clone(), answer);
//     Ok(warp::reply::with_status("Answer added", StatusCode::OK))
// }

pub async fn add_answer(
    store: store::Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
