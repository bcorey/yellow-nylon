use sqlx::Sqlite;
use sqlx::sqlite::{SqlitePoolOptions, SqliteArguments};
use sqlx::query::Query;
use sqlx::Error;


pub async fn query_generic<'a>(path: String, query: Query<'a, Sqlite, SqliteArguments<'a>>) -> Result<u64, Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&path).await?;

    let rows_affected = query
        .execute(&pool)
        .await
        .unwrap();

    pool.close().await;

    Ok(rows_affected.rows_affected())
}