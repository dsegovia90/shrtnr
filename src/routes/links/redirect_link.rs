use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct QueryData {
    id: String,
}

#[derive(sqlx::FromRow)]
struct Link {
    url: String,
}

#[get("/{id}")]
pub async fn redirect_link(data: web::Path<QueryData>, pool: web::Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE id = $1")
        .bind(data.id.to_string())
        .fetch_one(&**pool)
        .await;

    match query {
        Ok(row) => HttpResponse::PermanentRedirect()
            .append_header(("Location", row.url))
            .finish(),
        Err(_) => HttpResponse::BadRequest().body("Link does not exist."),
    }
}
