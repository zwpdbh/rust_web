mod models;
use models::store::{self, delete_question};
use std::collections::HashMap;
use warp::{
    filters::body::BodyDeserializeError, filters::cors::CorsForbidden, http::Method,
    http::StatusCode, reject::Rejection, reply::Reply, Filter,
};

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<store::Error>() {
        let reply = warp::reply::with_status(error.to_string(), StatusCode::RANGE_NOT_SATISFIABLE);
        Ok(reply)
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let store = store::Store::new();
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type-invalid-one")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let route_for_questions = "questions";

    let get_questions = warp::get()
        .and(warp::path(route_for_questions))
        .and(warp::path::end())
        .and(warp::query::<HashMap<String, String>>())
        .and(store_filter.clone())
        .and_then(store::get_questions);

    let add_question = warp::post()
        .and(warp::path(route_for_questions))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(store::add_question);

    let update_question = warp::put()
        .and(warp::path(route_for_questions))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(store::update_question);

    let delete_question = warp::delete()
        .and(warp::path(route_for_questions))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
