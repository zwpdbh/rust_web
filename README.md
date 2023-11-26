# Chapter 03

## Testing CORS response

```shell
curl -X OPTIONS localhost:3030/questions \
  -H "Access-Control-Request-Method: PUT" \
  -H "Access-Control-request-Headers: content-type" \
  -H "Origin: https://not-origin.io" -verbose
```

# Chapter 04 -- Implement a RESTful API

## Q and A

- Why need to create filter ?
  - Each HTTP request runs through the filters we setup and adds or modifies the data along the way.
  - To handle state with Warp, we have to create a filter.

## Get questions from in-memory

### How to load json file as in memory data store

- Our in memory database is a `Store` which is a `HashMap<QuestionId, Question>`.
- Load `Question`s into `Store` from a json file using `serde_json`.
- Pass `Store` to our route handle.
  - Each HTTP request runs through the filters we set up and adds or modifies the data along the way.
  - Create `store_filter`: We will create a filter which holds our store, and pass it to each route we want to access it. (p86)
  - Apply `store_filter` to route handler.

### Parsing query parameters

- The goal is to be able to visit as: `localhost:3030/questions?start=201&end=400`
- This is done by adding an additional filter `warp::query()` to our existing route.
  - Follow compiler error to modify `get_questions` function.
  - The query parameter will be presented as `HashMap<String, String>`.
    - Get the expected key and value.
    - Parse the value from `String` into `usize`.
- Troubleshooting: `curl localhost:3030/questions?start=1&end=200` only show one pair of keys.
  - It only shows `params: {"start": "1"}`, where is the rest of query string?
  - That is caused by:
    - before: `.and(store_filter)`
    - after: `.and(store_filter.clone())`

### Returning custom errors

- Return a proper error to the person who made the HTTP request

  ```rust
  #[derive(Debug)]
  enum Error {
    ParseError(std::num:ParseIntError),
    MIssingParameters,
  }
  ```

- Every time we want our custom type to learn new tricks or play nicely with other
  frameworks, we can implement traits on it. For `Error`, we need to implement two new traits:
  - [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html), to format the error into string
  - `Reject`, so we can return it in Warp route handler.

### Add a `Pagination` struct

- Define function `extract_pagination`
- Improve the `get_questions` function
- Improve the `return_error` function to match new error.

### Others

- Review the relationship between route handler and filter.
- Review what is the differences between `Debug` and `Display`
- Review `map_err`.
- Review question mark (?)
  - In Rust, the question mark ? is used at the end of a function that returns a Result type.
  - When you use ? at the end of a function, it essentially means "if the result is an Err, return it from the current function; otherwise, unwrap and return the value inside the Ok variant."
  - It's a convenient way to express that you want to pass the error up the call stack unless everything is successful.

## POST, PUT, and DELETE questions

### POST

- Troubleshooting
  - why post to `http://localhost:3030/questions` shows `Route not found`?
