import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {SettingsIcon, TestTubeIcon, TimerResetIcon} from "lucide-react";
import {Badge} from "@/components/ui/badge.tsx";

interface TimerProps {
  onComplete?: () => void;
}

export const Timer: React.FC<TimerProps> = ({onComplete}) => {
  const [remainingTime, setRemainingTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);
  const [timerMode, setTimerMode] = useState<"Work" | "Rest">("Rest");

  const [minutes, seconds] = [Math.floor(remainingTime / 60), remainingTime % 60];


  useEffect(() => {
    initTimerMode();
    initTimerStatus();

    // 监听倒计时更新事件
    const unListenCountdownUpdate = listen("countdown-update", (event: any) => {
      setRemainingTime(event.payload as number);
    });

    // 监听倒计时完成事件
    const unListenCountdownComplete = listen("countdown-complete", (event) => {
      setTimerMode(event.payload as "Work" | "Rest");
      setIsRunning(false);
      onComplete?.();
    });

    // 清理监听器
    return () => {
      unListenCountdownUpdate.then(fn => fn());
      unListenCountdownComplete.then(fn => fn());
    };
  }, [onComplete]);

  const initTimerMode = () => {
    invoke("get_countdown_mode")
      .then((mode) => {
        console.log("Current timer mode:", mode);
        setTimerMode(mode as "Work" | "Rest");
      });
  };

  const initTimerStatus = () => {
    invoke("get_remaining_time")
      .then((remainingTime) => {
        setRemainingTime(remainingTime as number);
      });
    invoke("get_countdown_state")
      .then((isRunning) => {
        setIsRunning(isRunning as boolean);
      });
  };

  const startTimer = async (sec: number) => {
    if (isRunning) return;
    setIsRunning(true);
    try {
      await invoke("start_countdown", {seconds: sec});
    } catch (error) {
      console.error("Failed to start countdown:", error);
      setIsRunning(false);
    }
  };

  return (
    <div className="w-full p-2">
      <div className="px-2 py-6 mx-auto max-w-96 flex flex-col gap-2 items-center rounded-md bg-neutral-100 dark:bg-neutral-900">
        <Badge variant={timerMode === "Work" ? "default" : "destructive"}>
          {timerMode === "Work" ? "工作" : "休息"}
        </Badge>
        <div className="font-sans text-7xl font-bold py-2 text-center ">
          {String(minutes).padStart(2, "0")} : {String(seconds).padStart(2, "0")}
        </div>
      </div>
      <div className="pt-4 flex justify-center gap-2">
        <Button
          size="icon"
          onClick={() => startTimer(10)}
          disabled={isRunning}
          className={isRunning ? "cursor-auto" : "cursor-pointer"}
        >
          <TestTubeIcon/>
        </Button>
        <Button
          size="icon"
          onClick={() => startTimer(60)}
          disabled={isRunning}
          className={isRunning ? "cursor-auto" : "cursor-pointer"}
        >
          <SettingsIcon/>
        </Button>
        <Button
          size="icon"
          onClick={() => startTimer(60)}
          disabled={isRunning}
        >
          <TimerResetIcon/>
        </Button>
      </div>
    </div>
  );
};