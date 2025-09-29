use actix_web::{get, web, HttpResponse, Responder};

use crate::middleware::Claims;


#[get("/protected")]
pub async fn protected_route(claims: web::ReqData<Claims>) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"message": "Protected API Accessed", "user": claims.sub}))
}