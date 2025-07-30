pub async fn pgsql_client() -> Result<sqlx::PgPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::PgPool::connect(&url).await
}