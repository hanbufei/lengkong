import { invoke } from "@tauri-apps/api/core";

export const get_class_by_id = async (id:number) => {
    try {
        let res = invoke('get_class_by_id',{id:id});
        return res;
    } catch (error) {
        return error
    }
}