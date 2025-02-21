use crate::dao::label::Label;

#[tauri::command]
pub async fn get_label(id:u64) -> Result<Label, String>{
     //调用dao层
     let res = Label::fetch_one(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn list_label(page_size:u8,page_num:u64) -> Result<Vec<Label>, String>{
     //调用dao层
     let res = Label::list(page_size,page_num).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn add_label(label_name:String,detail:String) -> Result<u64, String>{
     //调用dao层
     let res = Label::add(label_name,detail).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn delete_label(id:u64) -> Result<bool, String>{
     //调用dao层
     let res = Label::delete(id).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}

#[tauri::command]
pub async fn edit_label(id:u64,label_name:String,detail:String) -> Result<bool, String>{
     //调用dao层
     let res = Label::edit(id,label_name,detail).await;
     match res {
         Ok(json) => Ok(json),
         Err(err) =>Err(err.to_string()),
     }
}