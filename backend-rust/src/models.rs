use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct User {
    pub id_users: i64,
    pub username_users: String,
    pub password_users: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: i64,
    pub username: String,
    pub token: String,
}


#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug,Clone ,Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,        
    pub username: String,
    pub exp: usize,     
}

#[derive(Serialize)]
pub struct BarangResponse {
    pub id_barang: i64,
    pub nama_barang: String,
    pub harga_barang: i64,
    pub stok_barang: i32,
    pub file_barang: Option<String>,
}