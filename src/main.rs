mod api;
mod models;
mod db;
mod repositories;
mod telemetry;
mod config;
mod errors;
mod actors;
mod middleware;

use actix_web::{ middleware::Logger, web, App,  HttpServer};
use dotenvy::dotenv;
use telemetry::{get_subscriber, init_subscriber};
use actix::{Actor};

use crate::{actors::queue_manager::JobQueue, db::create_pool};


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();

    let subscriber =  get_subscriber("SuperCaterDB".into(), "info".into(), || std::io::stdout());
    init_subscriber(subscriber);

    let queue_manager_addr = JobQueue::new().start();


    let config = config::Config::from_env().expect("Failed to load the configuration!");

    let pool  = create_pool(&config.database.connection_string())
        .await
        .expect("Failed to create Database Pool!");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations!");

    let address = format!("{}:{}", config.application.host, config.application.port);
    tracing::info!("Server Started at http://{}", address);
    

    HttpServer::new(move|| {
        App::new() 
        .app_data(web::Data::new(pool.clone()))
        .app_data(web::Data::new(queue_manager_addr.clone()))
        .wrap(Logger::default())
        .configure(api::config) 
    })
    .bind(address)?
    .run()
    .await
}