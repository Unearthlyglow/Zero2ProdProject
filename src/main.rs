use actix_web::{HttpRequest, HttpResponse, Responder};
use std::net::TcpListener;
use zero2prod::run;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    run(listener)?.await
}

//Stopped at

//Notes:
//5/11/23::
//1. Dev dependencies are used exclusively when running tests or examples
//they do not get included in the final aplication binary.
