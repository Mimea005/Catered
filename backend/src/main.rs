use std::net::{Ipv4Addr};
use rocket::{self, Config, fs};
use anyhow::Result;
use scopes::api;

mod scopes;

#[rocket::main]
async fn main() -> Result<()> {

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| {
            println!("PORT not defined, using 8080");
            String::from("8080")
        })
    .parse()?;

    let address = Ipv4Addr::new(0,0,0,0).into();

    let _server = rocket::build()
        .configure(Config {port, address , ..Config::debug_default()})
        .attach(api::Routes)
        .attach(api::dishes::Routes)
        .mount("/", fs::FileServer::new("./web", fs::Options::default()))
        .launch()
    .await?;

    Ok(())
}