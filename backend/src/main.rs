extern crate rocket;

use rocket::{get, launch, routes};
use rocket::serde::json::Json;

use shared::MessageDto;

#[get("/api/message")]
fn index() -> Json<MessageDto> {
    Json(MessageDto { text: "Hello from Backend".to_string() })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
