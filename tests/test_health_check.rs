#[cfg(test)]
mod tests {
    use actix_web::{http::Method, test, App};
    use shrtnr::api::health_check::health_check;

    #[actix_web::test]
    async fn test_post_success_body() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::with_uri("/health-check")
            .method(Method::GET)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(
            resp.status().is_success(),
            "Expected success on health-check."
        );
    }
}
