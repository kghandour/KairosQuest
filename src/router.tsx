import { Route, Routes } from "react-router-dom";
import StartScreen from "./features/start_screen/StartScreen";
import Tasks from "./features/tasks/Tasks";
import { useAppConfigContext } from "./shared/contexts/ConfigContext";
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

function AppRouter () {
  const [isFirstRun, setIsFirstRun] = useState(true)  
  const {appConfig} = useAppConfigContext();


    useEffect(() => {
        async function is_first_run() {
          setIsFirstRun(await invoke('check_first_run'))
        }

        is_first_run()
    })
    
    
    return (
        <div>
          <h1>{JSON.stringify(appConfig)}</h1>
          <Routes>
              <Route path="/start_screen" element={<StartScreen />}></Route>
              <Route path="/" element={isFirstRun ? <StartScreen /> : <Tasks />}></Route>
          </Routes>
        </div>
    )
}
 
export default AppRouter;