use crate::dao::class::Class;

#[tauri::command]
pub async fn get_class_by_id(id:u64) -> Result<Class, String>{
     //调用dao层
     let res = Class::fetch_one_by_id(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}