use actix_web::{web, web::Data, App, HttpServer};
use shrtnr::routes::{health_check::health_check, links};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
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
                    .route("/link/{uid}", web::get().to(links::get_link)),
            )
            .route("/{uid}", web::get().to(links::get_link))
            .app_data(data_pool.clone())
        // .service(web::get().to(links::get_link))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
