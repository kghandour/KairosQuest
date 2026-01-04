import { Route, Routes } from "react-router-dom";
import StartScreen from "./features/start_screen/StartScreen";
import Tasks from "./features/tasks/Tasks";
import { useAppConfigContext } from "./shared/contexts/ConfigContext";

function AppRouter () {
    const {appConfig} = useAppConfigContext();

    const is_first_run = () => {
      return Object.keys(appConfig).length == 0
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