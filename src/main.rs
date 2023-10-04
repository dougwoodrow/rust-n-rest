#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
        "timestamp": "2023-10-03T12:34:56Z"
    });

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/create_person", routes![create_person])
}
