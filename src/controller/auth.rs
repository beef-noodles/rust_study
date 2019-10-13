
use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use serde::{Deserialize, Serialize};
use model::auth::{LoginBaseInfo, generate_jwt};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtResponse {
  code: usize,
  data: String,
  message: String,
}

pub fn auth(baseInfo: web::Form<LoginBaseInfo>) -> Result<HttpResponse> {
  let payload = json!({
    "username": baseInfo.username,
    "role": "admin",
  });
  let baseInfo = LoginBaseInfo{
    username: baseInfo.username,
    password: baseInfo.password,
  };
  let header = json!({});
  let secret = "guzhongren";
  let jwt = generate_jwt(&baseInfo);
  Ok(HttpResponse::Ok().json(JwtResponse{
    code: 200,
    data: jwt,
    message: "成功了".to_string(),
  }))
}