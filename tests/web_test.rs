use rocket::local::blocking::Client;
use rocket::http::Status;
use blobbot_rust::app::HealthResponse;

#[test]
fn should_handle_request() {
    let client = Client::tracked(blobbot_rust::app::setup_rocket()).expect("rocket");

    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), Some("Hello World!".into()));
}

#[test]
fn should_return_health_status() {
    let client = Client::tracked(blobbot_rust::app::setup_rocket()).expect("rocket");

    let response = client.get("/health").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let health = response.into_json::<HealthResponse>().expect("Health Response");
    assert!(health.healthy);
}
