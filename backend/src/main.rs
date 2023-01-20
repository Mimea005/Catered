use std::env;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{HttpServer, App, web, middleware::{Logger, NormalizePath, TrailingSlash,}, cookie::Key};

pub mod scopes;
use scopes::api;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let port = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse()
    .expect("Failed to conert port to number");

    let log_env = env_logger::Env::new().filter("LOGLEVEL");

    env_logger::init_from_env(log_env);

    HttpServer::new(|| {
        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[1; 64]))
                .build()
            )
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .wrap(Logger::default())
            .service(web::scope("/api").configure(api::configure))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
