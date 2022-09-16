use actix_web::{web, web::Data, App, HttpServer};
use shrtnr::api::{health_check, links, redirect_link};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .connect("postgres://daniel:password@localhost:5432/shrtnr")
        .await
        .expect("Failed to connect to Postgres");

    let data_pool = Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/api/v1")
                    .service(health_check)
                    .service(links::create_link)
                    .service(links::get_link),
            )
            .service(redirect_link)
            .app_data(data_pool.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
