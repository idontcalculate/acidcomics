use sqlx::PgPool;

pub async fn smoke_test(pool: &PgPool) {
    let (one,): (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await
        .expect("DB smoke test failed");

    assert_eq!(one, 1);
}
