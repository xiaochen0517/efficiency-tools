import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import {invoke} from "@tauri-apps/api/core";
import {Button} from "@/components/ui/button.tsx";

interface TimerProps {
  onComplete?: () => void;
}

export const Timer: React.FC<TimerProps> = ({ onComplete }) => {
  const [remainingTime, setRemainingTime] = useState<number>(0);
  const [isRunning, setIsRunning] = useState<boolean>(false);

  useEffect(() => {
    // 监听倒计时更新事件
    const unListenCountdownUpdate = listen('countdown-update', (event: any) => {
      setRemainingTime(event.payload as number);
    });

    // 监听倒计时完成事件
    const unListenCountdownComplete = listen('countdown-complete', () => {
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
      await invoke('start_countdown', { seconds });
    } catch (error) {
      console.error('Failed to start countdown:', error);
      setIsRunning(false);
    }
  };

  return (
    <div className="timer">
      <div className="display">
        Time remaining: {remainingTime} seconds
      </div>
      <Button
        onClick={() => startTimer(60)}
        disabled={isRunning}
      >
        Start 60s Timer
      </Button>
    </div>
  );
};