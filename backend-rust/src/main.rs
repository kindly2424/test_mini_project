mod db;
mod handlers;
mod middleware;
mod models;

use crate::middleware::auth::jwt_auth;
use axum::{Router, http::{Method, header}, middleware::from_fn, routing::{delete, get, post, put}};
use db::connect_db;
use handlers::{
    auth::{login, register},
    barang::{create_barang, delete_barang, list_barang, update_barang,download_barang_file},
};
use tokio::net::TcpListener;
use tower_http::cors::{CorsLayer,AllowOrigin};
use std::time::Duration;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db = connect_db().await;

    let protected = Router::new()
        .route("/barang", post(create_barang))
        .route("/barang", get(list_barang))
        .route("/barang/:id", put(update_barang))
        .route("/barang/:id", delete(delete_barang))
         .route("/barang/file/:filename", get(download_barang_file))
        .layer(from_fn(jwt_auth));

    let app = Router::new()
        .route("/register/user", post(register))
        .route("/login/user", post(login))
        .merge(protected)
        .with_state(db);

          let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
        .max_age(Duration::from_secs(3600))
        .allow_credentials(true);

         let app = app.layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
