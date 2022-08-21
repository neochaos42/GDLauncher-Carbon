/* @refresh reload */
import { onMount } from "solid-js";
import { render } from "solid-js/web";
// import LogRocket from "logrocket";
import { Router, hashIntegration } from "@solidjs/router";
import App from "./app";
import Modals from "./Modals";
import "tailwindcss/tailwind.css";
import "@gd/ui/style.css";
import "./utils/napi";

// LogRocket.init("hadq3z/mytest");

render(() => {
  onMount(() => {
    window.removeLoading();
  });

  return (
    <>
      <Router source={hashIntegration()}>
        <App />
      </Router>
    </>
  );
}, document.getElementById("root") as HTMLElement);

render(() => {
  return (
    <Router source={hashIntegration()}>
      <Modals />
    </Router>
  );
}, document.getElementById("overlay") as HTMLElement);

console.log("ipcRenderer", window.ipcRenderer);

// Usage of ipcRenderer.on
window.ipcRenderer.on("main-process-message", (_event, ...args) => {
  console.log("[Receive Main-process message]:", ...args);
});