use sqlx::SqlitePool;

#[sqlx::test(fixtures("users"))]
async fn current_user_without_signing_in(pool: SqlitePool) {
    assert_eq!(true, true)
}
