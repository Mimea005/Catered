use actix_web::{web, get, Responder};


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(api_info)
        .service(
            web::scope("/dishes")
                .configure(dishes::configure)
        )
    ;
}

#[get("/")]
async fn api_info() -> &'static str {
"
This is Catered's first api (V1)
    
:>
"
}

mod dishes {
    use super::*;
    use actix_web::{Either, HttpResponse};
    use serde_json::json;
    use types::Dish;
    
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
    
    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg
            .service(get_menu)
            .service(get_dish)
        ;
    }

    #[get("/")]
    async fn get_menu() -> impl Responder {
        let dishes = MENU.clone()
            .map(|d| {
                json!({
                    "name": d.name,
                    "description": d.description
                })
            });
        
        web::Json(dishes)
    }

    #[get("/{name}/")]
    async fn get_dish(req: web::Path<String>) -> Either<web::Json<Dish<'static>>, HttpResponse> {
        let found = MENU.iter()
            .filter(|d| d.name == req.clone())
            .next();
        match found {
            Some(dish) => Either::Left(web::Json(dish.clone())),
            None => Either::Right(HttpResponse::NotFound().body("Not a dish").into())
        }
    }
}