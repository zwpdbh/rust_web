use crate::store::Store;
use crate::types::pagination;
use crate::types::pagination::Pagination;
use crate::types::question::{NewQuestion, Question};
// use handle_errors::Error;
use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;
// #[instrument]
// pub async fn get_questions(
//     params: HashMap<String, String>,
//     store: Store,
//     // id: String,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     // log::info!("{} Start querying questions", id);

//     info!("querying questions");
//     if !params.is_empty() {
//         let pagination = pagination::extract_pagination(params)?;
//         // log::info!("{} Pagination set: {:?}", id, pagination);
//         info!(pagination = true);
//         let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
//         let res = &res[pagination.start..pagination.end];
//         Ok(warp::reply::json(&res))
//     } else {
//         // log::info!("{} No pagination used", id);
//         info!(pagination = false);
//         let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
//         Ok(warp::reply::json(&res))
//     }
// }

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
    // id: String,
) -> Result<impl warp::Reply, warp::Rejection> {
    // log::info!("{} Start querying questions", id);

    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = pagination::extract_pagination(params)?;
    }

    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        // Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

// pub async fn add_question(
//     store: Store,
//     question: Question,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     println!("add new question: {:?}", question);

//     store
//         .questions
//         .write()
//         .await
//         .insert(question.id.clone(), question);

//     Ok(warp::reply::with_status(
//         "Question added",
//         warp::http::StatusCode::OK,
//     ))
// }

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

// pub async fn update_question(
//     id: i32,
//     store: Store,
//     question: Question,
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     match store.questions.write().await.get_mut(&QuestionId(id)) {
//         Some(q) => *q = question,
//         None => return Err(warp::reject::custom(Error::QuestionNotFound)),
//     }

//     Ok(warp::reply::with_status(
//         "Question updated",
//         warp::http::StatusCode::OK,
//     ))
// }

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

// pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
//     match store.questions.write().await.remove(&QuestionId(id)) {
//         Some(_) => Ok(warp::reply::with_status(
//             "Question deleted",
//             warp::http::StatusCode::OK,
//         )),
//         None => Err(warp::reject::custom(Error::QuestionNotFound)),
//     }
// }

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status(
        format!("Question {} deleted", id),
        StatusCode::OK,
    ))
}
