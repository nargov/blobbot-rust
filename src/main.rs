use blobbot_rust::app;

#[rocket::main]
async fn main() {
    app::start().await
}
