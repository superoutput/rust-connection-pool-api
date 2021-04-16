use actix_web::{get, HttpResponse, Responder};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Health {
    status: String,
    total_space: i32,
    free_space: i32
}

#[get("/health/")]
pub async fn health() -> impl Responder {
    let u = &Health {
        status: "UP".to_string(),
        total_space: 0,
        free_space: 0
    };
    HttpResponse::Ok().json(u)
}
