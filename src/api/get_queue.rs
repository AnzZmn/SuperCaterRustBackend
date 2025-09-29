use actix::Addr;
use actix_web::{get, web, HttpResponse, Responder};

use crate::actors::{messages::GetJobQueue, queue_manager::JobQueue};

#[get("/get_queue")]
pub async fn get_queue_api(get_queue_addr: web::Data<Addr<JobQueue>>) -> impl Responder {
    match get_queue_addr.send(GetJobQueue).await{
        Ok(v) => HttpResponse::Ok().json(v.unwrap()),
        Err(_) => HttpResponse::InternalServerError().body("JobQueue is not Responding!")

    }
}