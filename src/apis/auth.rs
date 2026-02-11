use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct LoginRequest {
    #[serde(rename = "loginId")]
    login_id: String,
    password: String,
    #[serde(rename = "applicationId")]
    application_id: String,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "refreshToken")]
    refresh_token: String,
    token: String,
}

const IAM_PUBLIC_API_KEY: &str = env!("VITE_IAM_PUBLIC_API_KEY");

#[wasm_bindgen]
pub async fn login(login_id: String, password: String) -> Result<JsValue, JsValue> {
    let client = reqwest::Client::new();
    let body = LoginRequest {
        login_id: login_id,
        password,
        application_id: "6bcb945a-8bef-4c59-afdd-9018e5daeb7c".to_string(),
    };
    let response = client
        .post("http://localhost:3000/auth-api/api/v2/auth/login")
        .header(reqwest::header::AUTHORIZATION, IAM_PUBLIC_API_KEY)
        .json(&body)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Request error: {}", e)))?;
    let data: LoginResponse = response
        .json()
        .await
        .map_err(|e| JsValue::from_str(&format!("Response parsing error: {}", e)))?;
    Ok(to_value(&data).unwrap())
}
