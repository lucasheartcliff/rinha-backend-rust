use actix_web::{App, HttpServer};
use std::net::SocketAddr;

mod controllers;
use controllers::person;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    HttpServer::new(|| App::new().service(person::echo).service(person::hello))
        .bind(&addr)?
        .run()
        .await
}
