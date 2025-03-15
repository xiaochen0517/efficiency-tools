import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {SettingsIcon} from "lucide-react";
import {Button} from "@/components/ui/button.tsx";
import {RadioGroup, RadioGroupItem} from "@/components/ui/radio-group";
import {Label} from "@/components/ui/label.tsx";
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import {load} from "@tauri-apps/plugin-store";
import {Input} from "@/components/ui/input.tsx";


interface SettingsButtonProps {
  isRunning: boolean;
  onSave: () => void;
}

type WxPusherConfigType = {
  time_mode: "Short" | "Medium" | "Long" | "Test";
  spt_token: string;
}

export default function SettingsButton({isRunning, onSave}: SettingsButtonProps) {

  const [isDevMode, setIsDevMode] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [timeMode, setTimeMode] = useState<"Short" | "Medium" | "Long" | "Test">("Short");
  const [sptToken, setSptToken] = useState<string>("");

  useEffect(() => {
    load("store.json", {autoSave: false})
      .then(async (store) => {
        await store.get<WxPusherConfigType>("wxpusher_config")
          .then((config) => {
            if (!config) return;
            console.log("config", config);
            setSptToken(config.spt_token);
            setTimeMode(config.time_mode);
          });
      });
    invoke("is_dev_mode")
      .then((isDevMode) => {
        setIsDevMode(isDevMode);
      });
  }, [isSettingsOpen]);

  const commitSettings = () => {
    console.log("commitSettings", timeMode);
    invoke("set_pomodoro_time_mode", {timeMode})
      .then(() => {
        setIsSettingsOpen(false);
        onSave();
      });
    load("store.json", {autoSave: false})
      .then(async (store) => {
        await store.set("wxpusher_config", {
          time_mode: timeMode,
          spt_token: sptToken,
        });
      });
  };

  return (
    <Dialog open={isSettingsOpen} onOpenChange={setIsSettingsOpen}>
      <DialogTrigger asChild>
        <Button
          size="icon"
          disabled={isRunning}
          className={isRunning ? "cursor-auto" : "cursor-pointer"}
        >
          <SettingsIcon/>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>设置</DialogTitle>
          <DialogDescription/>
        </DialogHeader>
        <div className="flex flex-col gap-4">
          <RadioGroup
            defaultValue={timeMode}
            onValueChange={(value) => setTimeMode(value as "Short" | "Medium" | "Long")}
          >
            {isDevMode ? <div className="flex items-center space-x-2">
              <RadioGroupItem value="Test" id="option-test"/>
              <Label htmlFor="option-test">测试时间；</Label>
            </div> : null}
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="Short" id="option-short"/>
              <Label htmlFor="option-short">短 25分钟工作；&nbsp;&nbsp;5分钟短休息；15分钟长休息；</Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="Medium" id="option-medium"/>
              <Label htmlFor="option-medium">中 45分钟工作；10分钟短休息；20分钟长休息；</Label>
            </div>
            <div className="flex items-center space-x-2">
              <RadioGroupItem value="Long" id="option-long"/>
              <Label htmlFor="option-long">长 60分钟工作；15分钟短休息；30分钟长休息；</Label>
            </div>
          </RadioGroup>
          <div className="flex flex-col gap-1">
            <div className="">WxPusher SPT 配置</div>
            <Input
              value={sptToken}
              onChange={(e) => setSptToken(e.target.value)}
              placeholder="SPT Token"
            />
          </div>
          <Button className="w-fit" onClick={commitSettings}>保存</Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}