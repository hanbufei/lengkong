import { invoke } from "@tauri-apps/api/core";
import dayjs from "dayjs";

export interface article{
    class_id:number,
    name: string,
    location?: string,
    come_from?: string,
    label_id_list?: string,
    last_img?: string,
    sum_txt?: string
}

export const get_article = async (id:number) => {
    try {
        let res = invoke('get_article',{id:id});
        return res;
    } catch (error) {
        return error
    }
}

export const list_article = async (pageSize:number,pageNum:number) => {
    try {
        let res = invoke('list_article',{pageSize:pageSize,pageNum:pageNum});
        return res;
    } catch (error) {
        return error
    }
}

export const add_article = async (a:article) => {
    // console.log(dayjs().format("YYYY-MM-DD HH:mm:ss"));//2025-02-21 09:58:15
    try {
        let res = invoke('add_article',{
            classId:a.class_id,
            name:a.name,
            createAt:dayjs().format("YYYY-MM-DD HH:mm:ss"),
            location:a.location?a.location:"",
            comeFrom:a.come_from?a.come_from:"",
            labelIdList:a.label_id_list?a.label_id_list:"",
            lastImg:a.last_img?a.last_img:"",
            sumTxt:a.sum_txt?a.sum_txt:"",
        });
        return res;
    } catch (error) {
        return error
    }
}

export const delete_article = async (id:number) => {
    try {
        let res = invoke('delete_article',{id:id});
        return res;
    } catch (error) {
        return error
    }
}

export const edit_article_text = async (id:number,key:string,value:string) => {
    try {
        let res = invoke('edit_article_text',{id,key,value});
        return res;
    } catch (error) {
        return error
    }
}