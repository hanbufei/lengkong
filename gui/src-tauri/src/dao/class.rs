use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::get_pool;

//类别表
#[derive(Serialize, Deserialize,Debug,FromRow)]
pub struct Class {
    pub id: u64,
    pub class_name: String,
    pub detail: String,
}

impl Default for Class {
    fn default() -> Self {
        Self { id: Default::default(), class_name: Default::default(), detail: Default::default() }
    }
}

impl Class {
    pub async fn fetch_one_by_id(id:u64) -> Result<Class, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res: Class = sqlx::query_as::<_,Class>(
            r#"
            SELECT `id`, `class_name`,`detail`
            FROM `class`
            WHERE `id` = ?
        "#,
        )
        .bind(id)
        .fetch_one(&pool)
        .await?;
        Ok(res)
        //序列化数据
        // let serialized = serde_json::to_string(&res)?;
        // Ok(serialized)
    }
}