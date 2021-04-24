use actix_web::{get, post, web, HttpResponse};
use chrono::{DateTime, Duration, Utc};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::structs::claim::Claims;
use crate::middlewares::auth::AuthorizationService;
use crate::config::CONFIG;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String
}

#[post("/token/")]
async fn token(json: web::Json<Login>) -> HttpResponse {
    if CONFIG.username == json.username 
    && CONFIG.password == to_sha256(&json.password) {
        let _secret_key = &CONFIG.secret_key;
        let key = _secret_key.as_bytes();
        let expired_hours = 24;
        let mut _date: DateTime<Utc> = Utc::now() + Duration::hours(expired_hours);
        let my_claims = Claims {
            sub: json.username.to_string(),
            exp: _date.timestamp() as usize,
        };
        let token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key),
        ).unwrap();

        HttpResponse::Ok().json(json!(
            {
                "message": "Generate Access Token Successfully",
                "data": {
                    "access_token": token,
                    "expires_in": expired_hours * 3600,
                    "token_type": "Bearer",
                    "scope": "read only",
                    // "refresh_token": "e6909468b2304ea5a31f202095e8b4fc"
                }
            }
        ))
    }else {
        HttpResponse::Unauthorized().json(json!(
            {
                "message": "username or password wrong",
                "data": null
            }
        ))
    }
    
}

#[get("/hash/{text}/")]
async fn hash(text: web::Path<String>) -> HttpResponse {
    let hash_pw = to_sha256(text.as_str());
    HttpResponse::Ok().body(hash_pw)
}

#[get("/auth/validate/")]
async fn validate_token(_: AuthorizationService) -> HttpResponse {
    HttpResponse::Ok().body("Congratulation! You are Authorized.")
}

fn to_sha256(text: &str) -> String {
    let mut sha = Sha256::new();
    sha.input_str(text);
    sha.result_str()
}