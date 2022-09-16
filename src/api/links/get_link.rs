use actix_web::{get, web, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct QueryData {
    id: String,
}

#[derive(sqlx::FromRow, Serialize)]
struct Link {
    id: String,
    url: String,
    created_at: DateTime<Utc>,
}

#[get("/link/{id}")]
pub async fn get_link(data: web::Path<QueryData>, pool: web::Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE id = $1")
        .bind(data.id.to_string())
        .fetch_one(&**pool)
        .await;

    match query {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
