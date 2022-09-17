#[cfg(test)]
mod tests {
    use actix_web::{http::Method, test, web::Data, App};
    use reqwest::StatusCode;
    use shrtnr::api::redirect_link;

    use crate::common::db_startup::DBStartup;

    #[actix_web::test]
    async fn test_get_redirected_failure_on_non_existing() {
        let mut db_setup = DBStartup::new().await;

        let data_pool = Data::new(db_setup.connection_pool.clone());
        let row_id = "123123";

        let app = test::init_service(App::new().app_data(data_pool).service(redirect_link)).await;
        let req = test::TestRequest::with_uri(&format!("/{}", row_id))
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "Expected status code 404. {:?}",
            resp.into_body()
        );

        db_setup.close().await;
    }
}
