import { createEffect, createSignal, Match, Switch } from "solid-js";
import { add_article, edit_article_text, list_article } from "src/data/article";
import { add_class, delete_class, edit_class, get_class, list_class } from "src/data/class";

export default function Main({sendMessage}){
  const [menu, setMenu] = createSignal("home");//当前菜单
  const [text,setText] = createSignal("");
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
            <button onClick={()=>list_article(10,1)
              .then((res) => setText(JSON.stringify(res)))
              .catch((error) => console.log(error,"text-orange-500"))}>
                list_class
              </button>
              {text()}
            </Match>
            <Match when={menu() == "label"}>
              <button onClick={()=>edit_article_text(1,"name","1111")
              .then((res) => setText(JSON.stringify(res)))
              .catch((error) => sendMessage(error,"text-red-500"))}>
                test
              </button>
              {text()}
            </Match>
          </Switch>
        </div>
    </div>
  );
};