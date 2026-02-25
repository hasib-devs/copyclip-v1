// import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ClipboardProvider } from "./contexts/clipboard-context";
import { GamepadProvider } from "./contexts/gamepad-context";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  // <React.StrictMode>
  <ClipboardProvider>
    <GamepadProvider>
      <App />
    </GamepadProvider>
  </ClipboardProvider>,
  // </React.StrictMode>,
);
