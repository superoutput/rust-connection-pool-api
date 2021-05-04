use actix_web::{delete, get, patch, post, put, web, HttpRequest, HttpResponse};
use crate::middlewares::auth::AuthorizationService;
use crate::services::service_locator::{Connection, IdleConnection};


#[post("/http/")]
async fn create(_: AuthorizationService, _body: web::Bytes) -> HttpResponse {
    let _json: serde_json::Value = serde_json::from_str(std::str::from_utf8(&_body).unwrap()).expect("JSON was not well-formatted");
    let _connection = IdleConnection {};
    let _result = _connection.write(_json);
    match _result {
        Ok(_) => HttpResponse::Ok().json(_result.unwrap()),
        Err(_) => HttpResponse::InsufficientStorage()
            .json(_result.unwrap_err()),
    }
}

#[get("/http/{id}/")]
async fn get(_: AuthorizationService, _path: web::Path<String>) -> HttpResponse {
    let _json: serde_json::Value = serde_json::json!({"id": _path.to_string()});
    let _connection = IdleConnection {};
    let _result = _connection.read(_json);
    match _result {
        Ok(_) => HttpResponse::Ok().json(_result.unwrap()),
        Err(_) => HttpResponse::InternalServerError()
            .json(_result.unwrap_err()),
    }
}

#[get("/http/search/")]
async fn search(_: AuthorizationService, _req: HttpRequest) -> HttpResponse {
    let _json: serde_json::Value = serde_json::from_str(_req.query_string()).expect("JSON was not well-formatted");
    let _connection = IdleConnection {};
    let _result = _connection.read(_json);
    match _result {
        Ok(_) => HttpResponse::Ok().json(_result.unwrap()),
        Err(_) => HttpResponse::InternalServerError()
            .json(_result.unwrap_err()),
    }
}

#[put("/http/{id}/")]
async fn update(_: AuthorizationService, _path: web::Path<String>, _body: web::Bytes) -> HttpResponse {
    let _json: serde_json::Value = serde_json::from_str(std::str::from_utf8(&_body).unwrap()).expect("JSON was not well-formatted");
    let _connection = IdleConnection {};
    let _result = _connection.write(_json);
    match _result {
        Ok(_) => HttpResponse::Ok().json(_result.unwrap()),
        Err(_) => HttpResponse::InsufficientStorage()
            .json(_result.unwrap_err()),
    }
}

#[patch("/http/{id}/")]
async fn partial_update(_: AuthorizationService, _path: web::Path<String>, _body: web::Bytes) -> HttpResponse {
    let _json: serde_json::Value = serde_json::from_str(std::str::from_utf8(&_body).unwrap()).expect("JSON was not well-formatted");
    let _connection = IdleConnection {};
    let _result = _connection.write(_json);
    match _result {
        Ok(_) => HttpResponse::Ok().json(_result.unwrap()),
        Err(_) => HttpResponse::InsufficientStorage()
            .json(_result.unwrap_err()),
    }
}

#[delete("/http/{id}/")]
async fn delete(_: AuthorizationService, _path: web::Path<String>) -> HttpResponse {
    let _json: serde_json::Value = serde_json::json!({"id": _path.to_string()});
    let _connection = IdleConnection {};
    let _result = _connection.write(_json);
    match _result {
        Ok(_) => HttpResponse::Ok().json(_result.unwrap()),
        Err(_) => HttpResponse::InsufficientStorage()
            .json(_result.unwrap_err()),
    }
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(create);
    cfg.service(get);
    cfg.service(search);
    cfg.service(update);
    cfg.service(partial_update);
    cfg.service(delete);
}
