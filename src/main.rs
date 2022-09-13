use actix_web::{App, HttpServer, web::Data};
use sqlx::postgres::PgPoolOptions;
use shrtnr::routes::hello::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://daniel@localHost:5432/shrtnr")
        .await;

    let data_pool = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .app_data(data_pool.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
