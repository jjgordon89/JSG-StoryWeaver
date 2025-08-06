import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles/focus-mode.css";
import StateSynchronizer from "./components/common/StateSynchronizer";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <StateSynchronizer />
    <App />
  </React.StrictMode>,
);
