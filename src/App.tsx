import "./App.css";

import { useState } from "react";
// import { invoke } from "@tauri-apps/api/core";

function App() {
  const [consentedtoblowuptheirpc, consentplshahathx] = useState(false);

  return (
    <div className="content">
      <h1 className="center">hi welcome to pipebomb!!</h1>
      <p className="center">ok so what this is about is to pretty much nuke your whole main directory of your computer, yes you heard me right.</p>

      <label className="center">
        <input type="checkbox" checked={consentedtoblowuptheirpc} onChange={(val) => consentplshahathx(val.target.checked)}></input>
        i absolutely consent to this and i know that im gonna kill my computer including my files at any point
      </label>

      <button disabled={!consentedtoblowuptheirpc}>pls blow up my pc already :(</button>
    </div>
  );
}

export default App;
