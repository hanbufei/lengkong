import { invoke } from "@tauri-apps/api/core";

export const get_class_list = async () => {
    invoke('get_class_list').then((message) => console.log(message));
}