// keypoint: mod vs use
// The mod keyword tells our compiler the path to a module, and it stores it for future use.
// The use keyword uses the module and tells the compiler this: a module is available, and here is the path to it so I
// can use it in this file.
mod routes;
mod store;
mod types;
use std::collections::HashMap;
use warp::{http::Method, Filter};
// mod error;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let log = warp::log::custom(|info| {
        eprintln!(
            "{}{}{}{:?} from {} with {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed(),
            info.remote_addr().unwrap(),
            info.request_headers()
        );
    });

    let store = store::Store::new();
    // To handle state with Wrap, we have to create a filter, which holds our store, and pass it to each route we want to access it.
    // With warp::any, the any filter will match any request, so this statement will match any and all requests.
    // Call map on the filter to pass a value to the receiving function.
    // Move means the capture is done by value: move the values into the closure and takes ownership of them.
    // Now, store_filter could be applied to the route handler.
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
        .and_then(routes::question::get_questions);

    let add_question = warp::post()
        .and(warp::path(route_for_questions))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path(route_for_questions))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path(route_for_questions))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        // For application/x-www-form-urlencoded
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(delete_question)
        .with(log)
        .with(cors)
        .recover(handle_errors::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
