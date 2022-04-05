mod web_index;
mod web_map;
mod bean;

use web_index::index;
use web_map::login;

use actix_web::{web, App, HttpServer};



#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}