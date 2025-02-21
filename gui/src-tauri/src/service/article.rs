use crate::dao::article::Article;

#[tauri::command]
pub async fn get_article(id:u64) -> Result<Article, String>{
     //调用dao层
     let res = Article::fetch_one(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn list_article(page_size:u8,page_num:u64) -> Result<Vec<Article>, String>{
     //调用dao层
     let res = Article::list(page_size,page_num).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn add_article(
    class_id:u64,
    name: String,
    location: String,
    come_from: String,
    label_id_list: String,
    last_img: String,
    sum_txt: String) -> Result<u64, String>{
    //  let t = SqlDateTime::parse_from_str(&create_at, "%Y-%m-%d %H:%M:%S");
    //调用dao层
    let res = Article::add(class_id,name,location,come_from,label_id_list,last_img,sum_txt).await;
    match res {
        Ok(json) => Ok(json),
        Err(err) =>Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn delete_article(id:u64) -> Result<bool, String>{
     //调用dao层
     let res = Article::delete(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn edit_article_text(id:u64,key:String,value:String) -> Result<bool, String>{
    let editable_key_list = vec!["name".to_string(), "location".to_string(), "come_from".to_string(),"label_id_list".to_string(),"last_img".to_string(),"sum_txt".to_string()];
    if !editable_key_list.contains(&key){
        return Err("数据库不存在该列".to_string())
    }
     //调用dao层
     let res = Article::edit(id,key,value).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}