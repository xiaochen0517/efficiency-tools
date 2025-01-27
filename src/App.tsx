import "./App.css";
import {Button} from "@/components/ui/button.tsx";
import {Timer} from "@/view/Timer.tsx";
import {useEffect} from "react";
import {TimerIcon} from "lucide-react";

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
    <div className="w-screen h-screen overflow-y-auto flex flex-col gap-2 items-center dark:bg-neutral-950 p-2">
      <Button size="icon" variant="default">
        <TimerIcon/>
      </Button>
      <Timer onComplete={handleTimerComplete}/>
    </div>
  );
}

export default App;
