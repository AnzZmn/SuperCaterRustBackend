use actix::{Addr};
use actix_web::{get, web, HttpResponse, Responder};

use crate::actors::{messages::Participant, queue_manager::JobQueue};


#[get("/queue_demo")]
pub async fn add_to_queue(job_queue_addr: web::Data::<Addr<JobQueue>>) -> impl Responder {
    let new_participant = Participant{
      name: String::from("Anas Zaman P A"),
      contact: String::from("+91 8078010930"),
      id: String::from("")
    };

    job_queue_addr.do_send(new_participant);

    HttpResponse::Ok().body("Successfully Added a new Participant to Queue")
}