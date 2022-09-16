#[cfg(test)]
mod tests {
    use crate::common::db_startup::DBStartup;
    use actix_web::{http::Method, test, web::Data, App};
    use reqwest::StatusCode;
    use shrtnr::api::get_link::get_link;

    #[actix_web::test]
    async fn test_get_link_successfully() {
        let db_setup = DBStartup::new_faulty_db().await;

        let data_pool = Data::new(db_setup.connection_pool.clone());
        let row_id = "123123";

        let app = test::init_service(App::new().app_data(data_pool).service(get_link)).await;
        let req = test::TestRequest::with_uri(&format!("/link/{}", row_id))
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            StatusCode::INTERNAL_SERVER_ERROR,
            "Expected success."
        );
    }
}
