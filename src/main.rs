mod middlewares;
mod structs;
mod health;
mod file;

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
                .service(health::health)
                .service(file::upload)
                .service(file::download)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}