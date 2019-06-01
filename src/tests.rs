use rocket::http::Status;
use rocket::local::Client;
use rocket::http::ContentType;

fn index_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![super::index])
}

fn stream_auth_rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![super::stream_auth])
}

#[test]
fn index_returns_nothing() {
    let rocket = index_rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Nothing".to_string()));
}

#[test]
fn stream_auth_returns_404() {
    let rocket = stream_auth_rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client
        .post("/stream-auth")
        .body("name=test&psk=test")
        .header(ContentType::Form)
        .dispatch();

    assert_eq!(response.status(), Status::NotFound);
}

#[test]
fn stream_auth_returns_created() {
    let rocket = stream_auth_rocket();
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client
        .post("/stream-auth")
        .body("name=jon&psk=password")
        .header(ContentType::Form)
        .dispatch();

    assert_eq!(response.status(), Status::Created);
}
