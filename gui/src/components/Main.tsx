import { createEffect } from "solid-js";
import { get_class_list } from "src/data/commands";

export default function Main({sendMessage}){
  return (
    <div class="h-full w-full flex flex-row items-start justify-start">
        <div class={"basis-1/9 min-w-12 transform duration-300 left-0 top-0 h-full pt-10 border-black border-opacity-60 bg-black bg-opacity-50"}>
          菜单
        </div>
        <div class={"h-full w-full overflow-auto pt-10 pl-12 pr-4 bg-gray-100"}>
          主页面
          <button onClick={()=>sendMessage("success","text-teal-500")}>success</button>
          <button onClick={()=>sendMessage("text-orange-500","text-orange-500")}>text-orange-500</button>
          <button onClick={()=>sendMessage("text-red-500","text-red-500")}>text-red-500</button>
          <button onClick={get_class_list}>get_class_list</button>
        </div>
    </div>
  );
};