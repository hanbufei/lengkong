import { invoke } from "@tauri-apps/api/core";

export const get_label = async (id:number) => {
    try {
        let res = invoke('get_label',{id:id});
        return res;
    } catch (error) {
        return error
    }
}

export const list_label = async (pageSize:number,pageNum:number) => {
    try {
        let res = invoke('list_label',{pageSize:pageSize,pageNum:pageNum});
        return res;
    } catch (error) {
        return error
    }
}

export const add_label = async (labelName:string,detail:string) => {
    try {
        let res = invoke('add_label',{labelName:labelName,detail:detail});
        return res;
    } catch (error) {
        return error
    }
}

export const delete_label = async (id:number) => {
    try {
        let res = invoke('delete_label',{id:id});
        return res;
    } catch (error) {
        return error
    }
}

export const edit_label = async (id:number,labelName:string,detail:string) => {
    try {
        let res = invoke('edit_label',{id:id,labelName:labelName,detail:detail});
        return res;
    } catch (error) {
        return error
    }
}