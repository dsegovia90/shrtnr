use actix_web::{get, http::header::HeaderMap, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::{Map, Value};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct QueryData {
    id: String,
}

#[allow(dead_code)]
struct Link {
    id: String,
    url: String,
    created_at: DateTime<Utc>,
}

#[get("/{id}")]
pub async fn redirect_link(
    data: web::Path<QueryData>,
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let query = sqlx::query_as!(
        Link,
        "SELECT * FROM links WHERE id = $1",
        data.id.to_string()
    )
    .fetch_one(&**pool)
    .await;

    let head = req.head();
    let peer_addr = match head.peer_addr {
        Some(val) => val.to_string(),
        None => "".to_string(),
    };

    let analytics_query = sqlx::query!(
        r#"
                INSERT INTO link_hits (
                    headers,
                    method,
                    uri,
                    version,
                    peer_addr,
                    connection_type,
                    created_at,
                    link_id
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        convert_into_value(req.headers()),
        head.method.to_string(),
        head.uri.to_string(),
        format!("{:?}", head.version),
        peer_addr,
        format!("{:?}", head.connection_type()),
        Utc::now(),
        data.id.to_string()
    )
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
