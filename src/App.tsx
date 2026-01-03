import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [config, setConfig] = useState("");
  
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  async function get_config() {
    setConfig(await invoke("get_config"));
  }

  async function update_first_use(){
    setConfig(await invoke("update_first_use"));
  }
  
  return (
    <main className="container">
      <h1>KairosQuest</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>

      <div className="config-section">
        <h2>Configuration Settings</h2>

     </div>

      <div className="row">
        <button onClick={get_config}>Get Config</button>
      </div>
      <div className="row">
        <button onClick={update_first_use}>Update first use</button>
      </div>
      <p>{config}</p>
    </main>
  );
}

export default App;
