import React, { useContext, useEffect, useState } from "react";
import { Route, Routes } from "react-router-dom";
import StartScreen from "./features/start_screen/StartScreen";
import Tasks from "./features/tasks/Tasks";
import ConfigContext from "./shared/contexts/ConfigContext";

function AppRouter () {
    const {appConfig, setAppConfig} = useContext(ConfigContext);

    const is_first_run = () => {
      if('first_run' in appConfig){
        return appConfig["first_run"];
      }
      return true;
    }
    
    return (
        <div>
          <h1>{JSON.stringify(appConfig)}</h1>
          <Routes>
              <Route path="/start_screen" element={<StartScreen />}></Route>
              <Route path="/" element={is_first_run() ? <StartScreen /> : <Tasks />}></Route>
          </Routes>
        </div>
    )
}
 
export default AppRouter;