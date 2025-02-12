import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {PauseIcon, PlayIcon} from "lucide-react";
import {Badge} from "@/components/ui/badge.tsx";
import SettingsButton from "@/components/SettingsButton.tsx";

interface TimerProps {
  onComplete?: () => void;
}

type PomodoroStateType = {
  countdown_mode: "Work" | "Rest";
  remaining_time: number;
  rest_count: number;
  started: boolean;
  time_mode: "Test" | "Short" | "Medium" | "Long";
}

export const Timer: React.FC<TimerProps> = ({onComplete}) => {
  const [countdownMode, setCountdownMode] = useState<"Work" | "Rest">("Rest");
  const [remainingTime, setRemainingTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [restCount, setRestCount] = useState<number>(0);
  const [timeMode, setTimeMode] = useState<"Test" | "Short" | "Medium" | "Long">("Short");

  const [minutes, seconds] = [Math.floor(remainingTime / 60), remainingTime % 60];


  useEffect(() => {
    initPomodoroState();

    const unListenCountdownStarted = listen("countdown-started", () => {
      setIsRunning(true);
    });
    const unListenCountdownStopped = listen("countdown-stopped", () => {
      setIsRunning(false);
    });

    // 监听倒计时更新事件
    const unListenCountdownUpdate = listen("countdown-update", (event: any) => {
      setRemainingTime(event.payload as number);
    });

    // 监听倒计时完成事件
    const unListenCountdownComplete = listen("countdown-complete", () => {
      initPomodoroState();
      onComplete?.();
    });

    // 清理监听器
    return () => {
      unListenCountdownStarted.then(fn => fn());
      unListenCountdownStopped.then(fn => fn());
      unListenCountdownUpdate.then(fn => fn());
      unListenCountdownComplete.then(fn => fn());
    };
  }, [onComplete]);

  const initPomodoroState = () => {
    invoke<PomodoroStateType>("get_pomodoro_state")
      .then((state) => {
        console.log("当前番茄钟状态:", state);
        setCountdownMode(state.countdown_mode);
        setRemainingTime(state.remaining_time);
        setIsRunning(state.started);
        setRestCount(state.rest_count);
        setTimeMode(state.time_mode);
      });
  };

  const switchTimer = async () => {
    try {
      if (isRunning) {
        await invoke("stop_pomodoro");
      } else {
        await invoke("start_pomodoro");
      }
    } catch (error) {
      console.error("Failed to start countdown:", error);
    }
  };

  return (
    <div className="w-full p-2">
      <div className="px-2 py-6 mx-auto max-w-96 flex flex-col gap-2 items-center rounded-md bg-neutral-100 dark:bg-neutral-900">
        <div className="flex gap-2">
          <Badge variant={countdownMode === "Work" ? "default" : "destructive"}>
            {countdownMode === "Work" ? "工作" : "休息"}
          </Badge>
          <Badge variant="default">
            周期：{restCount}
          </Badge>
          <Badge variant="default">
            模式：{timeMode === "Test" ? "测试" : timeMode === "Short" ? "短" : timeMode === "Medium" ? "中" : "长"}
          </Badge>
        </div>
        <div className="font-sans text-7xl font-bold py-2 text-center ">
          {String(minutes).padStart(2, "0")} : {String(seconds).padStart(2, "0")}
        </div>
      </div>
      <div className="pt-4 flex justify-center gap-2">
        <SettingsButton isRunning={isRunning} onSave={initPomodoroState}/>
        <Button
          size="icon"
          onClick={switchTimer}
        >
          {isRunning ? <PauseIcon/> : <PlayIcon/>}
        </Button>
      </div>
    </div>
  );
};