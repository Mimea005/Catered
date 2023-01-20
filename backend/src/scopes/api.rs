use actix_web::web;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(api_info))
    ;
}


async fn api_info() -> &'static str {
    "This is Catered's first api (V1)\n:>"
}