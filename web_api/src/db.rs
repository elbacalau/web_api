use sqlx::{mysql::MySqlPool, MySql, Pool};
use std::env;

pub type DbPool = Pool<MySql>;

pub async fn init_pool() -> Result<DbPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL debe estar definida en las variables de entorno");

    let pool = MySqlPool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
