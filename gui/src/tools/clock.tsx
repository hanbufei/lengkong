import { createSignal, onCleanup } from "solid-js";

export default function Clock () {
    const month_list = ["1月", "2月", "3月", "4月", "5月", "6月", "7月", "8月", "9月", "10月", "11月", "12月"];
    const day_list = ["周日", "周一", "周二", "周三", "周四", "周五", "周六"];
    const getTime = () => {
        let current_time = new Date();
        let day = day_list[current_time.getDay()];
        let hour = current_time.getHours();
        let minute: string = current_time.getMinutes().toLocaleString();
        let month = month_list[current_time.getMonth()];
        let date = current_time.getDate().toLocaleString();
        let meridiem = (hour < 12 ? "上午" : "下午");
        if (minute.length === 1) {
            minute = "0" + minute
        }
        return day + " " + month + date + "日 " + hour + ":" + minute + " " + meridiem;
    }
    const [display_time, settime] = createSignal(getTime());//当前显示的内容

    const interval = setInterval(() => {
        settime(getTime());
      }, 10 * 1000);

    onCleanup(() => clearInterval(interval));

    return(
        <span>{display_time()}</span>
    )
}