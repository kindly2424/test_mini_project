use axum::{Json, extract::State, http::StatusCode};
use sqlx::MySqlPool;

use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{PasswordHash, SaltString, rand_core::OsRng};

use crate::models::{ApiResponse, Claims, LoginResponse, LoginUser, RegisterUser, User};

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use std::env;
pub async fn register(
    State(db): State<MySqlPool>,
    Json(payload): Json<RegisterUser>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    let username = payload.username.to_lowercase();

    let exists: i64 =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE username_users = ?")
            .bind(&username)
            .fetch_one(&db)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::<()> {
                        success: false,
                        message: "DB Error".into(),
                        data: None,
                    }),
                )
            })?;

    if exists > 0 {
        return Err((
            StatusCode::CONFLICT,
            Json(ApiResponse::<()> {
                success: false,
                message: "username sudah terdaftar".into(),
                data: None,
            }),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()> {
                    success: false,
                    message: "Hash gagal".into(),
                    data: None,
                }),
            )
        })?
        .to_string();

    sqlx::query("INSERT INTO users (username_users, password_users) VALUES (?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::<()> {
                    success: false,
                    message: "gagal tambah user".into(),
                    data: None,
                }),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::<()> {
            success: true,
            message: "Register Berhasil, silahkan login".into(),
            data: None,
        }),
    ))
}

pub async fn login(
    State(db): State<MySqlPool>,
    Json(payload): Json<LoginUser>,
) -> Result<(StatusCode, Json<ApiResponse<LoginResponse>>), (StatusCode, Json<ApiResponse<()>>)> {
    let username = payload.username.to_lowercase();

    let user = sqlx::query_as::<_, User>(
        "SELECT id_users, username_users, password_users FROM users WHERE username_users = ?",
    )
    .bind(&username)
    .fetch_one(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                success: false,
                message: "Username atau password Salah".into(),
                data: None,
            }),
        )
    })?;

    let parsed_hash = PasswordHash::new(&user.password_users).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: "Hash Error".into(),
                data: None,
            }),
        )
    })?;

    let argon2 = Argon2::default();

    if argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                success: false,
                message: "Username atau password Salah".into(),
                data: None,
            }),
        ));
    }

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id_users,
        username: user.username_users.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: "gagal genrate jwt token".to_string(),
                data: None,
            }),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: "Login berhasil".into(),
            data: Some(LoginResponse {
                id: user.id_users,
                username: user.username_users,
                token: token,
            }),
        }),
    ))
}
