use actix_multipart::Multipart;
use actix_web::{get, post, web, HttpResponse, Error};
// use futures::stream::once;
// use futures::future::ok;
use futures::{StreamExt, TryStreamExt};
use serde::{Serialize, Deserialize};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};


const UPLOAD_PATH: &str = "../storage/upload";


#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

#[post("/files/")]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    std::fs::create_dir_all(UPLOAD_PATH)?;
    let mut filename = "".to_string();
    while let Ok(Some(mut field)) = payload.try_next().await {
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
pub async fn download(filename: web::Path<String>) -> HttpResponse {
    let path = format!("{}/{}", UPLOAD_PATH, filename.to_string());
    if !std::path::Path::new(path.as_str()).exists() {
        return HttpResponse::NotFound().json(&File {
            name: filename.to_string(),
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            err: "file does not exists".to_string()
        });
    }

    let data = std::fs::read(path).unwrap();
    HttpResponse::Ok()
        .header("Content-Disposition", format!("form-data; filename={}", filename.to_string()))
        .body(data)
}
