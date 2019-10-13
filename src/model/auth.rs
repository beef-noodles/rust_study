extern crate frank_jwt;
use frank_jwt::{ encode, Algorithm};
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginBaseInfo {
  username: String,
  password: String,
}

pub fn generate_jwt(baseInfo: &LoginBaseInfo) -> Result<String> {
  let payload = json!({
    "username": baseInfo.username,
    "role": "admin",
  });
  let header = json!({});
  let secret = "guzhongren";
  let jwt = encode(header, &secret.to_string(), &payload, Algorithm::HS256).unwrap();
  Ok(jwt)
}
