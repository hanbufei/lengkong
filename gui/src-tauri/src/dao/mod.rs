use chrono::Local;
use sqlx::{MySqlPool,mysql::MySqlConnectOptions};

pub mod class;
pub mod label;
pub mod article;

pub type SqlDateTime = chrono::NaiveDateTime;

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

// 获取当前本地时间
pub fn local_now() -> SqlDateTime {
    Local::now().naive_local()
}