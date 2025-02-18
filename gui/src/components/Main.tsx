import { createEffect, createSignal, Match, Switch } from "solid-js";
import { get_class_list } from "src/data/commands";

export default function Main({sendMessage}){
  const [menu, setMenu] = createSignal("home");//当前菜单
  return (
    <div class="h-full w-full flex flex-row items-start justify-start">
        <div class='h-full pt-4 basis-1/4 flex flex-col space-y-6 justify-start items-center select-none z-40 '>
          <button onClick={()=>setMenu("home")} class="border-b-2 hover:border-b-orange-500 text-xl">文章</button>
          <button onClick={()=>setMenu("class")} class="border-b-2 hover:border-b-orange-500 text-xl">类别</button>
          <button onClick={()=>setMenu("label")} class="border-b-2 hover:border-b-orange-500 text-xl">标签</button>
          <a href="/" class="border-b-2 hover:border-b-orange-500 text-xl text-red-700">注销</a>
        </div>
        <div class={"h-full w-full overflow-auto pt-10 pl-12 pr-4 bg-gray-100"}>
          <Switch fallback={
            <a href="/new" class="border-b-2 hover:border-b-orange-500">新增</a>
          }>
            <Match when={menu() == "class"}>
              <button onClick={()=>sendMessage("text-orange-500","text-orange-500")}>text-orange-500</button>
            </Match>
            <Match when={menu() == "label"}>
              <button onClick={()=>sendMessage("text-red-500","text-red-500")}>text-red-500</button>
            </Match>
            <Match when={menu() == "class"}>
              <button onClick={()=>sendMessage("success","text-teal-500")}>success</button>
              <button onClick={get_class_list}>get_class_list</button>
            </Match>
          </Switch>
        </div>
    </div>
  );
};