pub mod add_queue;
pub mod get_queue;
pub mod gen_jwt;
pub mod protected;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::{api::{add_queue::add_to_queue, gen_jwt::init_jwt_gen, get_queue::get_queue_api, protected::protected_route}, middleware::validator};


#[get("/health")]
async fn health_check()-> impl Responder {
    HttpResponse::Ok().body("Ok")
}



pub fn config(cfg: &mut web::ServiceConfig) {

    let auth = HttpAuthentication::bearer(validator);

    cfg.service(health_check);
    cfg.service(add_to_queue);
    cfg.service(get_queue_api);
    cfg.service(init_jwt_gen);
    cfg
     .service(
        
        web::scope("/req_auth")
            .wrap(auth)
            .service(protected_route)
     );
}

