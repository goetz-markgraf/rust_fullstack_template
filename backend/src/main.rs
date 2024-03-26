#[macro_use]
extern crate rocket;

use rocket::{get, launch, routes};

#[get("/")]
fn index() -> String {
    shared::get_greeting()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
