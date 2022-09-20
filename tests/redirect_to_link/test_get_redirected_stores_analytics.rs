#[cfg(test)]
mod tests {
    use actix_web::{http::Method, test, web::Data, App};
    use chrono::Utc;
    use shrtnr::api::redirect_link;

    #[derive(sqlx::FromRow, Debug)]
    struct AnalyticsRow {
        link_id: String,
        headers: serde_json::Value,
    }

    use crate::common::db_startup::DBStartup;

    #[actix_web::test]
    async fn test_get_redirected_stores_analytics() {
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

        let app = test::init_service(App::new().app_data(data_pool).service(redirect_link)).await;
        let req = test::TestRequest::with_uri(&format!("/{}", row_id))
            .method(Method::GET)
            .append_header(("header-test", "123"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection(), "Expected success.");

        let query = sqlx::query_as::<_, AnalyticsRow>("SELECT * FROM link_hits WHERE link_id = $1")
            .bind(row_id)
            .fetch_one(&db_setup.connection_pool.clone())
            .await
            .expect("Could not query for analytics.");

        println!("{:?}", query);

        assert_eq!(
            query.headers.get("header-test").unwrap(),
            "123",
            "Expected headers to match."
        );
        assert_eq!(
            query.link_id, row_id,
            "Expected link_id to be equal to row_id."
        );

        db_setup.close().await;
    }
}
