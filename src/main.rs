#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::State;
use rocket::http::RawStr;
use rocket::response::Redirect;
use rocket_contrib::json::{Json, JsonValue};

use std::collections::HashMap;
use std::sync::Mutex;
use rand::{distributions::Alphanumeric, Rng};


// TODO parse environment variables for deployment URL
// TODO add HTTP response types for failed URLs

// declare the type for our temporary, in-memory datastore
type URLMap = HashMap<String, String>;
type SharedURLMap = Mutex<HashMap<String, String>>;

// Add URL Request Struct
#[derive(Deserialize, Serialize)]
struct ShortUrl {
    shortcode: Option<String>,
    destination: String,
}

/*
* Random String Generation
*/
fn generate_random_shortcode() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
    s
}


/*
* Server Methods
*/


/*
 * Stateful Methods - Require Map/DB Access
*/

#[get("/404")]
fn custom_error_404() -> &'static str {
    "URL not found!"
}

#[get("/<short>")]
fn index_echo(short: &RawStr, urls: State<SharedURLMap>) -> Redirect {

    // get map access
    let local_urls = urls.lock().expect("Could not get a Lock on URL Hashmap!");
    let data = local_urls.get(&short.to_string());

    println!("Fetching URL For {}", short);

    // lookup the Map to see if the URL with the specific shortcode exists
    match data {
        Some(url) => {
            println!("Found URL For Shortcode {} => {}", short, url);
            println!("Response URL is {}", url.as_str());

            // clone here so we don't directly refer to the map value
            // lifetime problem of returning a dropped value solved!
            let url_resp = url.clone();

            // BUG the URL added NEEDS to have https:// or http:// before it, or else it will 404
            Redirect::to(url_resp)
        }
        None => {
            println!("Nothing Found!");
            // TODO replace this with a nice response page
            Redirect::to("/404")
        }
    }

}

#[post("/", format = "json", data = "<new_url>")]
fn index_post(new_url: Json<ShortUrl>, urls: State<SharedURLMap>) -> JsonValue {

    // in the rare event we have the exact same random code generated twice, 
    // we can try to generate the shortcode again
    // let mut is_generated_sc = false;

    let shortcode: String = match &new_url.shortcode {
        Some(custom_shortcode) => {
            custom_shortcode.to_string()
        }
        None => {
            // is_generated_sc = true;
            generate_random_shortcode()
        }
    };

    // time
    let mut urls_local = urls.lock().unwrap();

    if urls_local.contains_key(&shortcode) {
        json!({
            "message": "Shortcode Already Exists!. Please Update!"
        })
    } else {

        // insert into the hashmap 
        // TODO is the cast for destination necessary?
        let shortcode_copy = shortcode.clone();
        urls_local.insert(shortcode, new_url.destination.to_string());

        println!("New URL > {} with shortcode {}", new_url.destination, shortcode_copy);

        json!({
            "message": "Successfully Added URL",
            "new_shortcode": shortcode_copy
        })
    }
}

#[get("/")]
fn index_get(urls: State<SharedURLMap>) -> JsonValue {

    // get all the URLs
    let local_urls = urls.lock().expect("Could not get a Lock on URL Hashmap!");

    // read adn respond
    json!(local_urls.clone())

}

#[delete("/<short>")]
fn index_delete(short: &RawStr, urls: State<SharedURLMap>) -> JsonValue {

    let mut local_urls = urls.lock().expect("Could not get a Lock on URL Hashmap!");
    let data = local_urls.get(&short.to_string());

    println!("Fetching URL For {}", short);

    // lookup the Map to see if the URL with the specific shortcode exists
    match data {
        Some(url) => {

            println!("Found URL For Shortcode {} => {}", short, url);
            println!("Response URL is {}", url.as_str());

            let local_short = short.to_string();

            // delete it
            local_urls.remove(&local_short);

            json!({
                "message": "Successfully Deleted!"
            })
        }
        None => {
            println!("Nothing Found!");
            // TODO replace this with a nice response page
            json!({
                "status": "error",
                "message": "Shortcode Does Not Exist!"
            })
        }
    }
}

#[patch("/<short>", format = "json", data="<updated_url_body>")]
fn index_update(short: &RawStr, urls: State<SharedURLMap>, updated_url_body: Json<ShortUrl>) -> JsonValue {

    let mut local_urls = urls.lock().expect("Could not get a Lock on URL Hashmap!");
    let data = local_urls.get(&short.to_string());

    println!("Fetching URL For {}", short);

    // lookup the Map to see if the URL with the specific shortcode exists
    match data {
        Some(url) => {

            println!("Found URL For Shortcode {} => {}", short, url);
            println!("Response URL is {}", url.as_str());

            let local_short = short.to_string();
            let local_new_url = updated_url_body.destination.to_string();

            // update it
            local_urls.insert(local_short, local_new_url);

            json!({
                "message": "Successfully Deleted!"
            })
        }
        None => {
            println!("Nothing Found!");
            // TODO replace this with a nice response page
            json!({
                "status": "error",
                "message": "Shortcode Does Not Exist!"
            })
        }
    }
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}


fn main() {
    rocket::ignite()
        .mount("/", routes![custom_error_404, index_get, index_echo, index_post, index_update, index_delete])
        .manage(Mutex::new(HashMap::<String, String>::new()))
        .register(catchers![not_found])
        .launch();
}
