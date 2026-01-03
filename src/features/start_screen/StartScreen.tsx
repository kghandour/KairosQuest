import { open } from '@tauri-apps/plugin-dialog';
import React, { useContext, useState } from 'react';
import ConfigContext from '../../shared/contexts/ConfigContext';
import { invoke } from '@tauri-apps/api/core';

function StartScreen(){
    const {appConfig, setAppConfig} = useContext(ConfigContext);
    const [workspacePath, setWorkspacePath] = useState("");

    async function selectDirectory(){
        const directory = await open({
            multiple: false,
            directory: true,
        });

        setWorkspacePath(directory || "");
        await update_config("workspace_path", directory)
    }

    async function update_config(key: string, value: string | boolean){
        let conf = {}
        conf[key] = value;
        setAppConfig(prevConfig => ({
            ...prevConfig,
            ...conf,
        }));

        await invoke('update_config_field', {key: key, field: value});
    }

    return(
        <div>
            <h1> Welcome to KairosQuest! </h1>
            <p> Start by selecting a workspace to store your files </p>
            <button onClick={selectDirectory}>Select Directory</button>
            <p>{workspacePath}</p>
        </div>
    )
}

export default StartScreen;