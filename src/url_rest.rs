use crate::security::check_auth;

use serde_json::{json, Value};
use warp::{Filter, reply::Json};

// implement a custom filter or set of methods for the URL API Methods

pub fn todos_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let urls_base = warp::path("urls");

    let list = urls_base
        .and(warp::get())
        .and(warp::path::end())
        .and(check_auth().untuple_one())
        .and_then(urls_list);

    let single_list = urls_base
        .and(warp::get())
        // add a path parameter - This type is inferred based on the first argument that the
        // handler function takes.
        // Parse Failures are 404s.
        .and(check_auth().untuple_one())
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(urls_single_list);

    let create_url = urls_base
        .and(warp::post())
        .and(check_auth().untuple_one())
        .and(warp::body::json())
        .and_then(urls_create);

    list.or(single_list).or(create_url)
}

async fn urls_list() -> Result<Json, warp::Rejection> {

    // TODO get from DB
    let todos = json!([
        {"id": "1", "url": "abc", "shortcode": "def"},
        {"id": "2", "url": "wxy", "shortcode": "xyz"},
    ]);

    let urls_json = warp::reply::json(&todos);
    Ok(urls_json)
}

async fn urls_single_list(shortcode: i64) -> Result<Json, warp::Rejection> {

    // TODO get single URL from the database
    let todos = json!([
        {"id": "1", "url": "https://www.google.com", "shortcode": shortcode},
    ]);

    let urls_json = warp::reply::json(&todos);
    Ok(urls_json)

}

async fn urls_create(req_body: Value) -> Result<Json, warp::Rejection> {

    // TODO add a new URL to the Database.
    let url_new = req_body;
    let url_json = warp::reply::json(&url_new);
    Ok(url_json)
}
