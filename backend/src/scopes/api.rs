use rocket::{fairing, Rocket, Build, routes, get};

pub struct Routes;

#[rocket::async_trait]
impl fairing::Fairing for Routes {
    fn info(&self) -> rocket::fairing::Info {
        fairing::Info {
            name: "api",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, mut rocket: Rocket<Build>) -> fairing::Result {

        rocket = rocket.mount("/api", routes![api_info]);

        fairing::Result::Ok(rocket)
    }
}

#[get("/")]
pub async fn api_info() -> &'static str {
"
This is Catered's first api (V1)
    
:>
"
}

pub mod dishes {
    use rocket::{async_trait, fairing::{Fairing, Info, Kind, self}, Rocket, Build, serde::json::Json, get, routes};
    use types::Dish;

    pub struct Routes;
    #[async_trait]
    impl Fairing for Routes {
        fn info(&self) -> Info {
            Info { name: "api:dishes", kind: Kind::Ignite }
        }

        async fn on_ignite(&self, mut rocket: Rocket<Build>) -> fairing::Result {

            rocket = rocket.mount("/api/dishes", routes![all_dishes]);
            

            fairing::Result::Ok(rocket)
        }
    }
    
    const MENU: [Dish; 2] = [
        types::Dish {
            name: "Bro ko",
            description: "A Brothers koli",
            price: (55, 99),
            publisher: "Kao"
        },
        types::Dish {
            name: "Brolognese",
            description: "Would you share with a bro?",
            price: (322, 49),
            publisher: "Kao"
        }
    ];

    #[get("/all")]
    async fn all_dishes() -> Json<[Dish<'static>; 2]> {
        Json::from(MENU)
    }
}