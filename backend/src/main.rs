use std::env;

use actix_web::{HttpServer, App, web};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let port = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse()
        .expect("Failed to conert port to number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn hello() -> &'static str {
    "Hello!"
}
