use actix_web::{get, http::header::HeaderMap, web, HttpRequest, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{Map, Value};
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
pub async fn redirect_link(
    data: web::Path<QueryData>,
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let query = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE id = $1")
        .bind(data.id.to_string())
        .fetch_one(&**pool)
        .await;

    let analytics_query =
        sqlx::query("INSERT INTO link_hits (headers, created_at, link_id) VALUES($1, $2, $3)")
            .bind(convert_into_value(req.headers()))
            .bind(Utc::now())
            .bind(data.id.to_string())
            .execute(&**pool)
            .await;

    match analytics_query {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    match query {
        Ok(row) => HttpResponse::TemporaryRedirect()
            .append_header(("Location", row.url))
            .finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

fn convert_into_value(headers: &HeaderMap) -> serde_json::Value {
    println!("he: {}", headers.len());
    let mut map = Map::new();
    for (header_name, header_value) in headers.into_iter() {
        let header_value_result = header_value.to_str();
        match header_value_result {
            Ok(val) => map.insert(header_name.to_string(), Value::String(val.to_string())),
            Err(_) => None,
        };
    }

    Value::Object(map)
}
