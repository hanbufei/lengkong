use sqlx::{MySqlPool,mysql::MySqlConnectOptions};

pub mod class;

//获取数据库连接池
pub async fn get_pool() -> Result<MySqlPool, sqlx::Error> {
    let config = MySqlConnectOptions::new()
        .host("localhost")
        .port(3306)
        .username("root")
        .password("lengkong#CMS1234")
        .database("cms");
    //新建mysql连接池
    let pool = MySqlPool::connect_with(config).await?;
    Ok(pool)
}