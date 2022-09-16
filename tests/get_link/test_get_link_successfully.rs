#[cfg(test)]
mod tests {
    use crate::common::db_startup::DBStartup;
    use actix_web::{http::Method, test, web::Data, App};
    use chrono::Utc;
    use shrtnr::api::get_link::get_link;

    #[actix_web::test]
    async fn test_get_link_successfully() {
        let mut db_setup = DBStartup::new().await;

        let data_pool = Data::new(db_setup.connection_pool.clone());
        let row_id = "123123";

        sqlx::query("INSERT INTO links (id, url, created_at) VALUES($1, $2, $3)")
            .bind(&row_id)
            .bind("www.google.com")
            .bind(Utc::now())
            .execute(&**data_pool)
            .await
            .expect("Wrote a link into the DB.");

        let app = test::init_service(App::new().app_data(data_pool).service(get_link)).await;
        let req = test::TestRequest::with_uri(&format!("/link/{}", row_id))
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success(), "Expected success.");

        db_setup.close().await;
    }
}
