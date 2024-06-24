use tokio::sync::OnceCell;

static DB: OnceCell<sqlx::Pool<sqlx::Sqlite>> = OnceCell::const_new();

pub async fn get() -> &'static sqlx::Pool<sqlx::Sqlite> {
    DB.get_or_init(|| async {
        sqlx::SqlitePool::connect("sqlite://prisma/test.db")
            .await
            .unwrap()
    })
    .await
}
