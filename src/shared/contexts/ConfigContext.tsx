import { invoke } from "@tauri-apps/api/core";
import { createContext, ReactNode, useContext, useEffect, useState } from "react";


interface AppConfigType {
  workspace_path: string;
}

interface AppContextInterface {
  appConfig: AppConfigType;
  setAppConfig: React.Dispatch<React.SetStateAction<AppConfigType>>;
}

const ConfigContext = createContext<AppContextInterface | null>(null);

export const ConfigProvider = ({children}: { children: ReactNode}) => {
  const [appConfig, setAppConfig] = useState<AppConfigType>({workspace_path: ""});

  useEffect(()=> {
    async function get_config() {
      setAppConfig(await invoke("get_all_config"));
    }

    get_config();
  }, []);

  return (
    <ConfigContext value={{appConfig, setAppConfig}}>
        {children}
    </ConfigContext>
  )
}

export default ConfigContext;

export const useAppConfigContext = () => {
  const context = useContext(ConfigContext);
  if (!context) {
    throw new Error('useAppConfig must be used within an AppProvider');
  }
  return context;
};