import Footer from '@components/Footer'
import Main from '@components/Main'
import {createEffect, createSignal,type Component } from 'solid-js'

const Admin: Component = () => {
  //全局变量
  const [delay, setDelay] = createSignal(5);//生命周期:每帧是5秒
  const [message, setMessage] = createSignal({ 
      text: "", 
      severity: "text-teal-500", //text-teal-500:成功 text-orange-500:警告 text-red-500:错误
    });
  const sendMessage = (text,severity) =>{setMessage({text, severity});setDelay(5)};

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
    <div class="w-screen h-screen flex flex-col">
        <Main sendMessage={sendMessage}/>
        <Footer message={message}/>
    </div>
  )
}

export default Admin
