#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;
use chrono::{Utc};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct ResponsePayload {
    message: String,
}

#[post("/", data = "<person>")]
fn create_person(person: Json<Person>) -> Json<serde_json::Value> {
    let response = json!({
        "message": format!("Hello, {} year old named {}!", person.age, person.name),
        "is_adult": person.age >= 18,
         // get current timestamp here
        "timestamp": Utc::now().to_rfc3339(),
    });

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/create_person", routes![create_person])
}
