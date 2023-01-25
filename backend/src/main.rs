use std::net::Ipv4Addr;

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
    let address = Ipv4Addr::new(0,0,0,0).into();

    let server_config = Config {
        port, address,
        ..Default::default()
    };

    let _server = rocket::build()
        .configure(server_config)
        .mount("/", routes![hello])
        .launch()
    .await?;

    Ok(())
}

#[get("/")]
fn hello() -> &'static str {
    "Helo world!"
}