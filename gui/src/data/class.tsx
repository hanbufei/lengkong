import { invoke } from "@tauri-apps/api/core";

export const get_class = async (id:number) => {
    try {
        let res = invoke('get_class',{id:id});
        return res;
    } catch (error) {
        return error
    }
}

export const list_class = async (pageSize:number,pageNum:number) => {
    try {
        let res = invoke('list_class',{pageSize:pageSize,pageNum:pageNum});
        return res;
    } catch (error) {
        return error
    }
}

export const add_class = async (className:string,detail:string) => {
    try {
        let res = invoke('add_class',{className:className,detail:detail});
        return res;
    } catch (error) {
        return error
    }
}

export const delete_class = async (id:number) => {
    try {
        let res = invoke('delete_class',{id:id});
        return res;
    } catch (error) {
        return error
    }
}

export const edit_class = async (id:number,className:string,detail:string) => {
    try {
        let res = invoke('edit_class',{id:id,className:className,detail:detail});
        return res;
    } catch (error) {
        return error
    }
}