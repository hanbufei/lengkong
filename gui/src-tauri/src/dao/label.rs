use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::get_pool;

//类别表
#[derive(Serialize, Deserialize,Debug,FromRow)]
pub struct Label {
    pub id: u64,
    pub label_name: String,
    pub detail: Option<String>,
}

impl Default for Label {
    fn default() -> Self {
        Self { id: Default::default(), label_name: Default::default(), detail: Default::default() }
    }
}

impl Label {
    async fn is_exist(label_name:String) -> Result<bool, Box<dyn std::error::Error>>{
        let pool: sqlx::Pool<sqlx::MySql> = get_pool().await?;
        //sql 查询
        let res: Result<Label, sqlx::Error> = sqlx::query_as(
            r#"
            SELECT `id`, `label_name`,`detail`
            FROM `label`
            WHERE `label_name` = ?
        "#,
        )
        .bind(label_name)
        .fetch_one(&pool)
        .await;
        match res {
            Ok(_) => return Ok(true),
            Err(sqlx::Error::RowNotFound) =>return Ok(false),
            Err(_) => return Ok(true)
        }
    }

    pub async fn fetch_one(id:u64) -> Result<Label, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res: Label = sqlx::query_as(
            r#"
            SELECT `id`, `label_name`,`detail`
            FROM `label`
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

    pub async fn list(page_size:u8,page_num:u64) -> Result<Vec<Label>, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res = sqlx::query_as(
           r#"
            SELECT `id`, `label_name`,`detail`
            FROM `label`
            ORDER by `id`
            LIMIT ? 
            OFFSET ?
        "#
        )
        .bind(page_size as i32)
        .bind((page_num-1) as i32 * page_size as i32)
        .fetch_all(&pool).await?;
        Ok(res)
    }

    pub async fn add(label_name:String,detail:String) -> Result<u64, Box<dyn std::error::Error>>{
        let exist = Self::is_exist(label_name.clone()).await?;
        if exist == false{
            let pool = get_pool().await?;
            let res = sqlx::query(
                r#"
                INSERT INTO `label` (`label_name`,`detail`)
                VALUES (?, ?)
            "#
            )
            .bind(label_name)
            .bind(detail)
            .execute(&pool).await?;
            return Ok(res.last_insert_id())
        }else {
            return Err("该条目已存在，请检查后重试！".into())
        }
    }

    pub async fn delete(id:u64) -> Result<bool, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res = sqlx::query(
           r#"
           DELETE FROM `label` 
           WHERE `id` = ?
        "#
        )
        .bind(id)
        .execute(&pool).await?;
        if res.rows_affected() == 0 {
            return Ok(false)
        }
        return Ok(true)
    }

    pub async fn edit(id:u64,label_name:String,detail:String) -> Result<bool, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res = sqlx::query(
           r#"
           UPDATE `label` 
           SET `label_name` = ?, `detail` = ?
           WHERE `id` = ?
        "#
        )
        .bind(label_name)
        .bind(detail)
        .bind(id)
        .execute(&pool).await?;
        if res.rows_affected() == 0 {
            return Ok(false)
        }
        return Ok(true)
    }
}