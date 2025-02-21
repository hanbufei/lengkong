use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::{get_pool, local_now, SqlDateTime};

//类别表
#[derive(Serialize, Deserialize,Debug,FromRow)]
pub struct Article {
    pub id: u64,
    pub class_id: u64,
    pub name: String, 
    pub create_at: SqlDateTime,
    pub location: Option<String>,
    pub come_from: Option<String>,
    pub modify_at: Option<SqlDateTime>,
    pub audit_at: Option<SqlDateTime>,
    pub label_id_list:Option<String>,
    pub last_img: Option<String>,
    pub sum_txt: Option<String>,
}

impl Default for Article {
    fn default() -> Self {
        Self { id: Default::default(), class_id: Default::default(), name: Default::default(), create_at: Default::default(), location: Default::default(), come_from: Default::default(), modify_at: Default::default(), audit_at: Default::default(), label_id_list: Default::default(), last_img: Default::default(), sum_txt: Default::default() }
    }
}

impl Article {
    async fn is_exist(name:String) -> Result<bool, Box<dyn std::error::Error>>{
        let pool: sqlx::Pool<sqlx::MySql> = get_pool().await?;
        //sql 查询
        let res: Result<Article, sqlx::Error> = sqlx::query_as(
            r#"
            SELECT *
            FROM `article`
            WHERE `name` = ?
        "#,
        )
        .bind(name)
        .fetch_one(&pool)
        .await;
        match res {
            Ok(_) => return Ok(true),
            Err(sqlx::Error::RowNotFound) =>return Ok(false),
            Err(_) => return Ok(true)
        }
    }

    pub async fn fetch_one(id:u64) -> Result<Article, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res: Article = sqlx::query_as(
            r#"
            SELECT *
            FROM `article`
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

    pub async fn list(page_size:u8,page_num:u64) -> Result<Vec<Article>, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        //sql 查询
        let res = sqlx::query_as(
           r#"
            SELECT *
            FROM `article`
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

    pub async fn add(
        class_id:u64,
        name: String,
        location: String,
        come_from: String,
        label_id_list: String,
        last_img: String,
        sum_txt: String) -> Result<u64, Box<dyn std::error::Error>>{
        let exist = Self::is_exist(name.clone()).await?;
        if exist == false{
            let pool = get_pool().await?;
            let res = sqlx::query(
                r#"
                INSERT INTO `article` (`class_id`,`name`,`create_at`,`location`,`come_from`,`label_id_list`,`last_img`,`sum_txt`)
                VALUES (?,?,?,?,?,?,?,?)
            "#
            )
            .bind(class_id)
            .bind(name)
            .bind(local_now())
            .bind(location)
            .bind(come_from)
            .bind(label_id_list)
            .bind(last_img)
            .bind(sum_txt)
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
           DELETE FROM `article` 
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

    pub async fn edit(id:u64,key:String,value:String) -> Result<bool, Box<dyn std::error::Error>>{
        let pool = get_pool().await?;
        let sql = format!("UPDATE `article` SET {} = ?, `modify_at` = ? WHERE `id` = ?",key);
        //sql 查询
        let res = sqlx::query(&sql)
        .bind(value)
        .bind(local_now())
        .bind(id)
        .execute(&pool).await?;
        if res.rows_affected() == 0 {
            return Ok(false)
        }
        return Ok(true)
    }
}