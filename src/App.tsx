import "./App.css";
import {Button} from "@/components/ui/button.tsx";
import {Timer} from "@/view/Timer.tsx";

function App() {

  const handleTimerComplete = () => {
    console.log("Timer completed!");
    // 在这里添加倒计时结束时要执行的操作
    alert("Countdown finished!");
  };

  return (
    <div>
      <Button variant="default">Test</Button>
      <Timer onComplete={handleTimerComplete}/>
    </div>
  );
}

export default App;
