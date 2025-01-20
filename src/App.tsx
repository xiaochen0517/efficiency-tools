import "./App.css";
import {Button} from "@/components/ui/button.tsx";
import {useEffect} from "react";

function App() {

  useEffect(() => {
    // 启动时默认将窗口调整到页面中心
    window.moveTo((window.screen.width - window.outerWidth) / 2, (window.screen.height - window.outerHeight) / 2);
  }, []);

  return (
    <Button variant="default">Test</Button>
  );
}

export default App;
