use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;

use crate::generate_random_short_id::get_random_short_id;

#[derive(Deserialize)]
pub struct PostData {
    url: String,
}

#[post("/link")]
pub async fn create_link(data: web::Json<PostData>, pool: web::Data<PgPool>) -> impl Responder {
    if data.url.is_empty() {
        return HttpResponse::BadRequest().body("url param must not be empty");
    }

    let url = data.url.to_string();

    if let Err(e) = reqwest::get(&url).await {
        return HttpResponse::BadRequest().body(e.to_string());
    }

    let random_id = get_random_short_id();

    let query = sqlx::query("INSERT INTO links (id, url, created_at) VALUES($1, $2, $3)")
        .bind(&random_id)
        .bind(&url)
        .bind(Utc::now())
        .execute(&**pool)
        .await;

    match query {
        Ok(_) => HttpResponse::Ok().body(random_id),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

// HttpResponse::PermanentRedirect()
//     .append_header(("Location", "https://www.google.com/"))
//     .finish()
