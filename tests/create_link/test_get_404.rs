#[cfg(test)]
mod tests {
    use actix_web::{http::Method, test, App};
    use reqwest::StatusCode;
    use shrtnr::api::create_link::create_link;

    #[actix_web::test]
    async fn test_get_404() {
        let app = test::init_service(App::new().service(create_link)).await;
        let req = test::TestRequest::with_uri("/link")
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status(),
            StatusCode::NOT_FOUND,
            "Expected get to be 404."
        );
    }
}
