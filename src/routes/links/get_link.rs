use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryData {
    uid: String,
}

pub async fn get_link(data: web::Path<QueryData>) -> impl Responder {
    HttpResponse::Ok().body(data.uid.to_string())
}
