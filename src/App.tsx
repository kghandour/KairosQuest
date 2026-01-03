import React, { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { BrowserRouter } from "react-router-dom";
import ConfigContext from "./shared/contexts/ConfigContext";
import AppRouter from "./router";

function App() {
  // Check if first run using config

  const [appConfig, setAppConfig] = useState({});

    

  useEffect(()=> {
    async function get_config() {
      setAppConfig(JSON.parse(await invoke("get_config")));
    }

    get_config();
  }, []);
  
  return (
    <BrowserRouter>
      <ConfigContext value={{appConfig, setAppConfig}}>
        <AppRouter />
      </ConfigContext>
    </BrowserRouter>
  );
}

export default App;
