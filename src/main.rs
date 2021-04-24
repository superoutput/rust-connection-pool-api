mod middlewares;
mod services;
mod structs;
mod health;
mod file;
mod config;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Service;
use futures::future::FutureExt;


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hi! I am R00† .")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .wrap_fn(|req, srv| {
                    println!("Hi from start. You requested: {}", req.path());
                    srv.call(req).map(|res| {
                        println!("Hi from response");
                        res
                    })
                })
                .service(index)
                .service(services::auth::token)
                .service(services::auth::validate_token)
                .service(services::auth::hash)
                .service(health::health)
                .service(file::upload)
                .service(file::download)
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}