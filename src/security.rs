use warp::Filter;

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn check_auth() -> impl Filter<Extract = ((),), Error = warp::Rejection> + Clone {
    // implement custom auth to check the header to see if we're authenticated our not
    // Auth header needs to be something along the lines of `123.exp.signature`
    // implement a blank filter
    warp::any()
        .and(warp::header::<String>(HEADER_XAUTH))
        .and_then(|xauth: String| async move {
            // trivial auth check
            if !xauth.ends_with(".exp.signature") {
                return Err(warp::reject::custom(FailAuth));
            }

            Ok::<(), warp::Rejection>(())

        })
}

// this is a custom error type, better to use this
// than to panic or do something similar
#[derive(Debug)]
pub struct FailAuth;
impl warp::reject::Reject for FailAuth {}
