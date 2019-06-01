#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use rocket::http::Status;
use rocket::request::Form;

#[get("/")]
fn index() -> &'static str {
    "Nothing"
}

#[derive(FromForm)]
struct AuthRequest {
  name: String,
  psk: String,
}

#[post("/stream-auth", data = "<request>")]
fn stream_auth(request: Form<AuthRequest>) -> Status {
    if request.name == "jon" && request.psk == "password" {
        Status::Created
    } else {
        Status::NotFound
    }
}

fn app() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, stream_auth])
}

fn main() {
  app().launch();
}
