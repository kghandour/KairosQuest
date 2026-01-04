import React, { useContext, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { BrowserRouter } from "react-router-dom";
import ConfigContext, { convertVectorToJson } from "./shared/contexts/ConfigContext";
import AppRouter from "./Router";

function App() {
  // Check if first run using config

  const [appConfig, setAppConfig] = useState({});

    

  useEffect(()=> {
    async function get_config() {
      setAppConfig(await invoke("get_all_config"));
    }

    get_config();
  }, []);
  
  return (
    <BrowserRouter>
      <ConfigContext.Provider value={{appConfig, setAppConfig}}>
        <AppRouter />
      </ConfigContext.Provider>
    </BrowserRouter>
  );
}

export default App;
