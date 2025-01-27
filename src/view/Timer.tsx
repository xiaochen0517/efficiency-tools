import {useEffect, useState} from "react";
import {listen} from "@tauri-apps/api/event";
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";
import {TimerResetIcon} from "lucide-react";

interface TimerProps {
  onComplete?: () => void;
}

export const Timer: React.FC<TimerProps> = ({onComplete}) => {
  const [remainingTime, setRemainingTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);

  useEffect(() => {
    // 监听倒计时更新事件
    const unListenCountdownUpdate = listen("countdown-update", (event: any) => {
      setRemainingTime(event.payload as number);
    });

    // 监听倒计时完成事件
    const unListenCountdownComplete = listen("countdown-complete", () => {
      setIsRunning(false);
      onComplete?.();
    });

    // 清理监听器
    return () => {
      unListenCountdownUpdate.then(fn => fn());
      unListenCountdownComplete.then(fn => fn());
    };
  }, [onComplete]);

  const startTimer = async (seconds: number) => {
    if (isRunning) return;

    setIsRunning(true);
    setRemainingTime(seconds);

    try {
      await invoke("start_countdown", {seconds});
    } catch (error) {
      console.error("Failed to start countdown:", error);
      setIsRunning(false);
    }
  };

  const [minutes, seconds] = [Math.floor(remainingTime / 60), remainingTime % 60];

  return (
    <div className="flex flex-col gap-2 items-center w-1/2 p-2 rounded-md bg-neutral-100 dark:bg-neutral-900">
      <div className="font-sans text-3xl font-bold py-2 text-center">
        {String(minutes).padStart(2, "0")} : {String(seconds).padStart(2, "0")}
      </div>
      <Button
        size="icon"
        onClick={() => startTimer(60)}
        disabled={isRunning}
      >
        <TimerResetIcon/>
      </Button>
    </div>
  );
};