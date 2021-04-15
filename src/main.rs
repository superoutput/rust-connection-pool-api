use actix_multipart::Multipart;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
// use futures::stream::once;
// use futures::future::ok;
use futures::StreamExt;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Write;

const UPLOAD_PATH: &str = "../storage/upload";

#[derive(Serialize, Deserialize)]
struct Health {
    status: String,
    total_space: i32,
    free_space: i32
}

#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

#[derive(Deserialize)]
struct Download {
    name: String
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hi! I am R00† .")
}

#[get("/health/")]
async fn health() -> impl Responder {
    let u = &Health {
        status: "UP".to_string(),
        total_space: 0,
        free_space: 0
    };
    HttpResponse::Ok().json(u)
}

#[post("/files/")]
async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    std::fs::create_dir_all(UPLOAD_PATH)?;
    let mut filename = "".to_string();
    while let Some(Ok(mut field)) = payload.next().await {
        let content_type = field.content_disposition().unwrap();
        filename = format!("{} - {}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros(), content_type.get_filename().unwrap(), );
        let filepath = format!("{}/{}", UPLOAD_PATH, sanitize_filename::sanitize(&filename));
        // File::create is blocking operation, use thread pool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, use thread pool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    // Result::<_, std::io::Error>::Ok(res)
    // or
    // std::io::Result::Ok(res)
    Ok(HttpResponse::Ok().json(&File {
        name: filename,
        time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        err: "".to_string(),
    }))
}

#[get("/files/{name}/")]
async fn download(info: web::Path<Download>) -> HttpResponse {
    let path = format!("{}/{}", UPLOAD_PATH, info.name.to_string());
    if !std::path::Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: info.name.to_string(),
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            err: "file does not exists".to_string()
        });
    }

    let data = std::fs::read(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", info.name.to_string()))
        .body(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .service(index)
                .service(health)
                .service(upload)
                .service(download)
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}