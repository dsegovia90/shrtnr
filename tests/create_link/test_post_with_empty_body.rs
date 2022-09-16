#[cfg(test)]
mod tests {
    use actix_web::{http::Method, test, App};
    use shrtnr::routes::create_link::create_link;

    #[actix_web::test]
    async fn test_post_with_empty_body() {
        let app = test::init_service(App::new().service(create_link)).await;
        let req = test::TestRequest::with_uri("/link")
            .method(Method::POST)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400, "Expected empty body to return 400.");
    }
}
