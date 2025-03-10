use crate::dao::class::Class;

#[tauri::command]
pub async fn get_class(id:u64) -> Result<Class, String>{
     //调用dao层
     let res = Class::fetch_one(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn list_class(page_size:u8,page_num:u64) -> Result<Vec<Class>, String>{
     //调用dao层
     let res = Class::list(page_size,page_num).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn add_class(class_name:String,detail:String) -> Result<u64, String>{
     //调用dao层
     let res = Class::add(class_name,detail).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn delete_class(id:u64) -> Result<bool, String>{
     //调用dao层
     let res = Class::delete(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn edit_class(id:u64,class_name:String,detail:String) -> Result<bool, String>{
     //调用dao层
     let res = Class::edit(id,class_name,detail).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}