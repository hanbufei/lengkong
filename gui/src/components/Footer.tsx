import { createEffect } from "solid-js";

export default function Footer({message}){
  return (
    <footer class="flex w-full bg-gray-300">
      <div class={"h-full w-full truncate "+ message().severity} >
        {message().text}
      </div>
      <div class="h-full w-full pr-2 place-content-center center text-end basis-1/4">&copy; 2025 冷空 v0.1</div>
    </footer>
  )
};