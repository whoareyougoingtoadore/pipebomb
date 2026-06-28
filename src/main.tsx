import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
    <footer>
      <p>a project made by <a href="https://github.com/whoareyougoingtoadore" target="_blank">me</a> at <a href="https://onekey.hackclub.com/" target="_blank">onekey</a></p>
    </footer>
  </React.StrictMode>,
);
