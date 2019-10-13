
use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::model::auth::{LoginBaseInfo, generate_jwt};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse {
  code: usize,
  data: String,
  message: String,
}

pub fn auth(base_info: web::Form<LoginBaseInfo>) -> Result<HttpResponse> {
  let payload = json!({
    "username": base_info.username.clone(),
    "role": "admin",
  });
  let jwt = generate_jwt(&payload);
  Ok(HttpResponse::Ok().json(JwtResponse{
    code: 200,
    data: jwt,
    message: "成功了".to_string(),
  }))
}