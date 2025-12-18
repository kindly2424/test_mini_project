use axum::{
    Json,
    body::Bytes,
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::Utc;
use sqlx::{MySqlPool, Row};
use std::{fs, path::PathBuf};
use tokio::fs as tokio_fs;

use crate::models::{ApiResponse, BarangResponse};

const UPLOAD_DIR: &str = "uploads";
const MAX_FILE_SIZE: usize = 5 * 1024 * 1024; // 5 MB
const ALLOWED_EXT: [&str; 6] = ["pdf", "jpg", "jpeg", "png", "docx", "txt"];

pub async fn create_barang(
    State(db): State<MySqlPool>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    fs::create_dir_all(UPLOAD_DIR).ok();

    let mut nama_barang = String::new();
    let mut harga_barang = 0;
    let mut stok_barang = 0;
    let mut temp_file_name: Option<String> = None;
    let mut temp_file_bytes: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("");

        match name {
            "nama_barang" => {
                nama_barang = field.text().await.unwrap_or_default();
            }
            "harga_barang" => {
                harga_barang = field.text().await.unwrap_or_default().parse().unwrap_or(0);
            }
            "stok_barang" => {
                stok_barang = field.text().await.unwrap_or_default().parse().unwrap_or(0);
            }
            "file_barang" => {
                let fname = match field.file_name().map(|s| s.to_string()) {
                    Some(f) if !f.trim().is_empty() => f,
                    _ => continue,
                };
                let ext = fname.rsplit('.').next().unwrap_or("").to_lowercase();

                if !ALLOWED_EXT.contains(&ext.as_str()) {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            success: false,
                            message: "Ekstensi file tidak diizinkan".into(),
                            data: None,
                        }),
                    ));
                }
                let bytes = field.bytes().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            success: false,
                            message: "Gagal membaca file".into(),
                            data: None,
                        }),
                    )
                })?;

                if bytes.len() > MAX_FILE_SIZE {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            success: false,
                            message: "Ukuran file maksimal 5MB".into(),
                            data: None,
                        }),
                    ));
                }

                if !bytes.is_empty() {
                    temp_file_name = Some(format!("{}_{}", Utc::now().timestamp(), fname));
                    temp_file_bytes = Some(bytes);
                }
            }
            _ => {}
        }
    }

    if nama_barang.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Nama barang wajib diisi".into(),
                data: None,
            }),
        ));
    }

    if harga_barang < 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Harga barang wajib lebih dari 0".into(),
                data: None,
            }),
        ));
    }

    if stok_barang < 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Stok barang wajib lebih dari 0".into(),
                data: None,
            }),
        ));
    }
    let mut file_barang: Option<String> = None;

    if let (Some(fname), Some(bytes)) = (temp_file_name, temp_file_bytes) {
        let path = format!("{}/{}", UPLOAD_DIR, fname);
        fs::write(&path, bytes).unwrap();
        file_barang = Some(fname);
    }

    sqlx::query(
        "INSERT INTO barang (nama_barang, harga_barang, stok_barang, file_barang)
         VALUES (?, ?, ?, ?)",
    )
    .bind(nama_barang)
    .bind(harga_barang)
    .bind(stok_barang)
    .bind(file_barang)
    .execute(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: "Gagal menambah barang".into(),
                data: None,
            }),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            success: true,
            message: "Barang berhasil ditambahkan".into(),
            data: None,
        }),
    ))
}

pub async fn list_barang(
    State(db): State<MySqlPool>,
) -> Result<Json<ApiResponse<Vec<BarangResponse>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let rows = sqlx::query(
        "SELECT id_barang, nama_barang, harga_barang, stok_barang, file_barang
         FROM barang WHERE deleted_at IS NULL",
    )
    .fetch_all(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: "Gagal ambil data".into(),
                data: None,
            }),
        )
    })?;

    let mut result = Vec::new();

    for row in rows {
        result.push(BarangResponse {
            id_barang: row.get("id_barang"),
            nama_barang: row.get("nama_barang"),
            harga_barang: row.get("harga_barang"),
            stok_barang: row.get("stok_barang"),
            file_barang: row.get("file_barang"),
        });
    }

    Ok(Json(ApiResponse {
        success: true,
        message: "Success".into(),
        data: Some(result),
    }))
}

pub async fn update_barang(
    State(db): State<MySqlPool>,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    fs::create_dir_all(UPLOAD_DIR).ok();

    let mut nama_barang: Option<String> = None;
    let mut harga_barang: Option<i64> = None;
    let mut stok_barang: Option<i32> = None;
    let mut file_barang: Option<String> = None;

    let old_file: Option<String> =
        sqlx::query_scalar("SELECT file_barang FROM barang WHERE id_barang = ?")
            .bind(id)
            .fetch_optional(&db)
            .await
            .map_err(|_| {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        success: false,
                        message: "Barang tidak ditemukan".into(),
                        data: None,
                    }),
                )
            })?
            .flatten();

    while let Some(field) = multipart.next_field().await.map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                message: "Multipart error".into(),
                data: None,
            }),
        )
    })? {
        match field.name().unwrap_or("") {
            "nama_barang" => {
                let txt = field.text().await.unwrap_or_default();
                if !txt.trim().is_empty() {
                    nama_barang = Some(txt);
                }
            }

            "harga_barang" => {
                let txt = field.text().await.unwrap_or_default();
                if !txt.trim().is_empty() {
                    if let Ok(val) = txt.parse::<i64>() {
                        harga_barang = Some(val);
                    }
                }
            }

            "stok_barang" => {
                let txt = field.text().await.unwrap_or_default();
                if !txt.trim().is_empty() {
                    if let Ok(val) = txt.parse::<i32>() {
                        stok_barang = Some(val);
                    }
                }
            }

            "file_barang" => {
                let fname = match field.file_name().map(|s| s.to_string()) {
                    Some(f) if !f.trim().is_empty() => f,
                    _ => continue,
                };

                let ext = fname.rsplit('.').next().unwrap_or("").to_lowercase();

                if !ALLOWED_EXT.contains(&ext.as_str()) {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            success: false,
                            message: "Ekstensi file tidak diizinkan".into(),
                            data: None,
                        }),
                    ));
                }

                let bytes = field.bytes().await.map_err(|_| {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            success: false,
                            message: "Gagal membaca file".into(),
                            data: None,
                        }),
                    )
                })?;

                if bytes.len() > MAX_FILE_SIZE {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            success: false,
                            message: "Ukuran file maksimal 5MB".into(),
                            data: None,
                        }),
                    ));
                }
                let filename = format!("{}_{}", Utc::now().timestamp(), fname);
                let path = format!("{}/{}", UPLOAD_DIR, filename);

                if !bytes.is_empty() {
                    if fs::write(&path, bytes).is_ok() {
                        file_barang = Some(filename);
                    }
                }
            }
            _ => {}
        }
    }

    sqlx::query(
        "UPDATE barang SET
         nama_barang = COALESCE(?, nama_barang),
         harga_barang = COALESCE(?, harga_barang),
         stok_barang = COALESCE(?, stok_barang),
         file_barang = COALESCE(?, file_barang)
         WHERE id_barang = ?",
    )
    .bind(nama_barang)
    .bind(harga_barang)
    .bind(stok_barang)
    .bind(file_barang.clone())
    .bind(id)
    .execute(&db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                message: "Gagal update barang".into(),
                data: None,
            }),
        )
    })?;

    if let (Some(new_file), Some(old)) = (file_barang, old_file) {
        if new_file != old {
            let old_path = format!("{}/{}", UPLOAD_DIR, old);
            let _ = fs::remove_file(old_path);
        }
    }

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: "Barang berhasil diupdate".into(),
            data: None,
        }),
    ))
}

pub async fn delete_barang(
    State(db): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), (StatusCode, Json<ApiResponse<()>>)> {
    sqlx::query("UPDATE barang SET deleted_at = NOW() WHERE id_barang = ?")
        .bind(id)
        .execute(&db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    success: false,
                    message: "Gagal hapus barang".into(),
                    data: None,
                }),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            success: true,
            message: "Barang dihapus".into(),
            data: None,
        }),
    ))
}

pub async fn download_barang_file(Path(filename): Path<String>) -> impl IntoResponse {
    let path = PathBuf::from(UPLOAD_DIR).join(&filename);

    if !path.exists() {
        return (StatusCode::NOT_FOUND, "File tidak ditemukan").into_response();
    }

    match tokio_fs::read(path).await {
        Ok(bytes) => axum::response::Response::builder()
            .status(StatusCode::OK)
            .header(
                axum::http::header::CONTENT_DISPOSITION,
                format!("inline; filename=\"{}\"", filename),
            )
            .body(axum::body::Body::from(bytes))
            .unwrap(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Gagal membaca file").into_response(),
    }
}
