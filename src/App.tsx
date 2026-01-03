import React from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { BrowserRouter } from "react-router-dom";
import AppRouter from "./router";

function App() {

  async function get_config() {
    const config = await invoke("get_config");
  }

  async function update_first_use(){
    await invoke("update_first_use");
  }
  
  return (
    <BrowserRouter>
      <AppRouter />
    </BrowserRouter>
  );
}

export default App;
