use rocket::{self, routes, get, Config};
use anyhow::Result;


#[rocket::main]
async fn main() -> Result<()> {

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| {
            println!("PORT not defined, using 8080");
            String::from("8080")
        })
    .parse()?;

    let _server = rocket::build()
        .configure(Config {port, ..Default::default()})
        .mount("/", routes![hello])
        .launch()
    .await?;

    Ok(())
}

#[get("/")]
fn hello() -> &'static str {
    "Helo world!"
}