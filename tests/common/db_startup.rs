use sqlx::{Connection, Executor, PgConnection, PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug)]
pub struct DBStartup {
    pub connection_pool: Pool<Postgres>,
    connection: PgConnection,
    test_db_name: String,
}

impl DBStartup {
    pub async fn new() -> Self {
        let test_db_name = format!("test-shrtnr-db-{}", Uuid::new_v4());
        let pg_url = "postgres://daniel:password@localhost:5432/";
        let mut connection = PgConnection::connect(pg_url)
            .await
            .expect("Failed to connect to Postgres");
        connection
            .execute(format!("CREATE DATABASE \"{}\"", test_db_name).as_str())
            .await
            .expect("Failed to create test db.");

        let connection_pool = PgPool::connect(format!("{}{}", pg_url, test_db_name).as_str())
            .await
            .expect("Failed to connect to test db.");
        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");

        DBStartup {
            connection_pool,
            connection,
            test_db_name,
        }
    }

    pub async fn new_faulty_db() -> Self {
        let pg_url = "postgres://daniel:password@localhost:5432/";
        let connection_pool = PgPool::connect(pg_url.to_string().as_str())
            .await
            .expect("Failed to connect to test db.");
        let connection = PgConnection::connect(pg_url)
            .await
            .expect("Failed to connect to Postgres");
        DBStartup {
            connection_pool,
            connection,
            test_db_name: "<purposefully_empty>".to_owned(),
        }
    }

    pub async fn close(&mut self) {
        self.connection_pool.close().await;
        self.connection
            .execute(format!("DROP DATABASE \"{}\"", self.test_db_name).as_str())
            .await
            .expect("Could not close connection with db.");
    }
}
