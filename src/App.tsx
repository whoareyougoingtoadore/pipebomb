import "./App.css";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function StinkyVM() {
  return (
    <div className="content">
      <h1>ur no fun</h1>
      <p>yeah i figured out you would be testing this out on a virtual machine, surprise surprise, i check that.</p>
      <p>well that goes the detection by defender just because i do vm checks for fun to force you to test in your machine, but ehhhhh its ok ig</p>
      <p>if you are forced to still try this project for going into the unified db, have fun compiling it, read the README!</p>
    </div>
  )
}

function ExplosionSounds() {
  return (
    <div className="content">
      <h1>congrants1!1!</h1>
      <a>your pipe bomb installation has successfully went through</a>
      <p>now do your daily stuff now! do <a href="https://monkeytype.com/" target="_blank">monkeytype</a> or something, we'll let you know when your PC gets nuked after pressing a key</p>
      <a>oh and if you chicken out, uh, you can close the window and make sure the process is killed for stopping this - yeah.</a>
    </div>
  )
}

function PlsDoNotBlowUpYourPCYetHaha({ onState }: { onState: () => void }) {
    const [consentedtoblowuptheirpc, consentplshahathx] = useState(false);

    return (
      <div className="content">
        <h1 className="center">hi welcome to pipebomb!!</h1>
        <p className="center">ok so what this is about is to pretty much nuke your whole main directory of your computer, yes you heard me right.</p>
        <p className="center">ok but like it doesnt nuke it instantly, however, every time you press a key there's a veryyyyy small chance that your whole root directory is deleted</p>

        <label className="center">
          <input type="checkbox" checked={consentedtoblowuptheirpc} onChange={(val) => consentplshahathx(val.target.checked)}></input>
          anyways, check this to absolutely consent to this and that you know your OS installation is gonna get screwed and so does your files too and you wont be able to recover them
        </label>

        <button className="center" disabled={!consentedtoblowuptheirpc} onClick={() => { onState(); }}>pls blow up my pc already :(</button>
      </div>
    )
}

function App() {

  const [loserisusingavm, setloserisusingavm] = useState<boolean | null>(null);
  const [entered, yesentered] = useState(false);

  useEffect(() => {
    invoke<boolean>("is_ts_a_vm")
      .then((goodboy) => setloserisusingavm(goodboy))
      .catch(() => setloserisusingavm(false)); // i guess bro
  }, []);

  if (loserisusingavm) {
    return <StinkyVM />
  }

  return entered
    ? <ExplosionSounds />
    : <PlsDoNotBlowUpYourPCYetHaha onState={() => { if (loserisusingavm == false) { yesentered(true) } } } />
}

export default App;
