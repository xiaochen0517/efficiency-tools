import "./App.css";
import {Timer} from "@/view/Timer.tsx";
import {useEffect} from "react";

function App() {

  useEffect(() => {
    // On page load or when changing themes, best to add inline in `head` to avoid FOUC
    document.documentElement.classList.toggle(
      "dark",
      localStorage.currentTheme === "dark" ||
      (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),
    );
    // Whenever the user explicitly chooses light mode
    localStorage.currentTheme = "light";
    // Whenever the user explicitly chooses dark mode
    localStorage.currentTheme = "dark";
    // Whenever the user explicitly chooses to respect the OS preference
    localStorage.removeItem("theme");
  });

  const handleTimerComplete = () => {
    console.log("Timer completed!");
    // 在这里添加倒计时结束时要执行的操作
    alert("Countdown finished!");
  };

  return (
    <div className="w-screen h-screen flex flex-col items-center justify-center overflow-y-auto dark:bg-neutral-950 p-2">
      <Timer onComplete={handleTimerComplete}/>
    </div>
  );
}

export default App;
