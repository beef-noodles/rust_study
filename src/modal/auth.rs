extern crate frank_jwt;
use frank_jwt::{ encode, Algorithm};
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse {
  code: usize,
  data: String,
  message: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
  username: String,
  password: String,
}

pub fn generate_jwt(login: web::Form<Login>) -> Result<HttpResponse> {
  let payload = json!({
    "username": login.username,
    "role": "admin",
  });
  let header = json!({});
  let secret = "guzhongren";
  let jwt = encode(header, &secret.to_string(), &payload, Algorithm::HS256).unwrap();
  Ok(HttpResponse::Ok().json(JwtResponse{
    code: 200,
    data: jwt,
    message: "成功了".to_string(),
  }))
}
