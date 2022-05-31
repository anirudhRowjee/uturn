mod security;
mod url_rest;
use warp::Filter;

const WEB_FOLDER: &str = "static/";

#[tokio::main]
async fn main() {


    /* Declaring Route Handlers and Patterns to Match Them to */
    let index = warp::path::end()
        .and(warp::get())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));

    let hi = warp::path("hi")
        .and(warp::get())
        .map(|| "Hello from Hi!");

    /* Declaring Collections of Routes */
    let staticfiles = warp::fs::dir(WEB_FOLDER);
    let apis = hi.or(url_rest::todos_filter());

    let static_site = index.or(staticfiles);

    // combine the two sets of routes
    let routes = static_site.or(apis);

    println!("starting the web server...");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
