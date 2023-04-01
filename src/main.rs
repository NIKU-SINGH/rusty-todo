mod models;
mod config;

use crate::models::status;
use actix_web::{HttpServer, App, web, Responder,HttpResponse};
use std::io;
use dotenv::dotenv;

async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(status { status:"Up".to_string()})
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();
    println!("Starting the web server at http://{}:{}/",config.server.host,config.server.port);
    // Create new HTTP server with application factory.
    // an application factory is a function or set of functions that creates and configures instances of an application
    HttpServer::new(|| {
        App::new()
            .route("/",web::get().to(status))
    })
    .bind(format!("{}:{}",config.server.host,config.server.port))? 
    .run()
    .await
}