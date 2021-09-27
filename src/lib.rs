#[macro_use]
extern crate rocket;


pub mod app {
    use rocket::{Build, Rocket};
    use rocket::serde::{Serialize, Deserialize};
    use rocket::serde::json::Json;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct HealthResponse {
        pub healthy: bool,
    }

    pub async fn start() {
        println!("Server starting");
        setup_rocket()
            .launch()
            .await
            .expect("error starting web")
    }

    #[get("/")]
    fn index() -> &'static str {
        "Hello World!"
    }

    #[get("/health")]
    fn health() -> Json<HealthResponse> {
        Json(HealthResponse{healthy: true})
    }

    pub fn setup_rocket() -> Rocket<Build> {
        rocket::build()
            .mount("/", routes![index, health])
    }
}