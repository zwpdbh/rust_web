// keypoint: mod vs use
// The mod keyword tells our compiler the path to a module, and it stores it for future use.
// The use keyword uses the module and tells the compiler this: a module is available, and here is the path to it so I
// can use it in this file.
mod routes;
mod store;
mod types;
use std::collections::HashMap;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
// mod error;

#[tokio::main]
async fn main() {
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    // let log = warp::log::custom(|info| {
    //     log::info!(
    //         "{} {} {} {:?} from {} with {:?}",
    //         info.method(),
    //         info.path(),
    //         info.status(),
    //         info.elapsed(),
    //         info.remote_addr().unwrap(),
    //         info.request_headers()
    //     );
    // });
    // step1: add the log level
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "practical_rust_book=info,warp=error".to_owned());

    let store = store::Store::new("postgres://postgres:postgres@localhost:5432/rustweb").await;
    // To handle state with Wrap, we have to create a filter, which holds our store, and pass it to each route we want to access it.
    // With warp::any, the any filter will match any request, so this statement will match any and all requests.
    // Call map on the filter to pass a value to the receiving function.
    // Move means the capture is done by value: move the values into the closure and takes ownership of them.
    // Now, store_filter could be applied to the route handler.
    let store_filter = warp::any().map(move || store.clone());
    // step2: set the tracing subscriber
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our routes' durations.
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // let id_filter = warp::any().map(|| uuid::Uuid::new_v4().to_string());

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
        // .and(id_filter)
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            // step3: set up logging for custom events.
            tracing::info_span!("get_questions request", method = %info.method(), path = %info.path(), id = %uuid::Uuid::new_v4())
        }));

    let add_question = warp::post()
        .and(warp::path(route_for_questions))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path(route_for_questions))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path(route_for_questions))
        .and(warp::path::param::<i32>())
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
        // .with(log)
        .with(cors)
        // step4: set up logging for incoming requests
        .with(warp::trace::request())
        .recover(handle_errors::return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
