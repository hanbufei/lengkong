import Footer from '@components/Footer'
import { useNavigate } from '@solidjs/router';
import {createEffect, createSignal,type Component } from 'solid-js'

const Login: Component = () => {
  //全局变量
  const [delay, setDelay] = createSignal(5);//生命周期:每帧是5秒
  const [message, setMessage] = createSignal({ 
      text: "", 
      severity: "text-teal-500", //text-teal-500:成功 text-orange-500:警告 text-red-500:错误
    });
  const sendMessage = (text,severity) =>{setMessage({text, severity});setDelay(5)};
  
  const [username, setUsername] = createSignal("");
  const [password, setPassword] = createSignal("");
  const navigate = useNavigate();

  const handleSubmit = (event) => {
    event.preventDefault();
    // 检查密码是否正确
    if (username()==="lk" && password() === "lk1234") {
      navigate("/admin", { replace: true }); // 导航到后台页面
    } else {
      sendMessage("错误 | 用户名或密码错误！","text-red-500");
    }
  };

  createEffect(()=>{
    message();
  });

  //生命周期
  setInterval(() => {
    setDelay(delay()-1);
    if(delay()<=0){
      sendMessage("","text-teal-500");//清空消息
      setDelay(5);
    };
  }, 1000);

  return (
    <div class="w-screen h-screen flex flex-col bg-[url('/img/bg.png')] bg-cover">
        <h1 class="pt-8 text-center text-3xl text-white">冷空CMS管理后台</h1>
        <div class="border-b-2 pt-4 border-b-gray-500"/>
        <form class="h-full w-full flex flex-col items-start text-white pl-8 pt-40">
          <div class="flex flex-row pt-2 ">
            <label>用户：</label>
            <input class="border-2 bg-gray-500" type="text" onChange={(e)=>setUsername(e.target.value)}>{username()}</input>
          </div>
          <div class="flex flex-row pt-2">
            <label>密码：</label>
            <input class="border-2 bg-gray-500" type="password" onChange={(e)=>setPassword(e.target.value)}>{password()}</input>
          </div>
          <div class="pt-4 pl-32">
            <button onClick={(e)=>handleSubmit(e)} class="border-b-2 hover:border-b-orange-500 text-xl text-center">登录</button>
          </div>
        </form>
        <Footer message={message}/>
    </div>
  )
}

export default Login
