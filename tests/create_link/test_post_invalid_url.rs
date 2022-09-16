#[cfg(test)]
mod tests {
    use crate::common::db_startup::DBStartup;
    use actix_web::{http::Method, test, web::Data, App};
    use serde_json::json;
    use shrtnr::routes::create_link::create_link;

    #[actix_web::test]
    async fn test_post_invalid_url() {
        let mut db_setup = DBStartup::new().await;

        let data_pool = Data::new(db_setup.connection_pool.clone());
        let app = test::init_service(App::new().app_data(data_pool).service(create_link)).await;
        let req = test::TestRequest::with_uri("/link")
            .method(Method::POST)
            .set_json(json!({
                "url": "some random url"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "Expected failure with invalid url.");

        db_setup.close().await;
    }
}
