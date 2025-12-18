use crate::models::{ApiResponse, Claims};
use axum::{
    Json,
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

use std::env;

pub async fn jwt_auth(mut req: Request<Body>, next: Next) -> Result<Response, Response> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let token = match auth_header {
        Some(v) => v.replace("Bearer ", ""),
        None => {
            let body = Json(ApiResponse::<()> {
                success: false,
                message: "Token tidak ditemukan".into(),
                data: None,
            });
            return Err((StatusCode::UNAUTHORIZED, body).into_response());
        }
    };

    let secret = env::var("JWT_SECRET").unwrap();

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(_) => {
            let body = Json(ApiResponse::<()> {
                success: false,
                message: "Token tidak valid".into(),
                data: None,
            });
            return Err((StatusCode::UNAUTHORIZED, body).into_response());
        }
    };

    req.extensions_mut().insert(token_data.claims);

    Ok(next.run(req).await)
}
