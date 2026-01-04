import { open } from '@tauri-apps/plugin-dialog';
import { useState } from 'react';
import { useAppConfigContext } from '../../shared/contexts/ConfigContext';
import { invoke } from '@tauri-apps/api/core';

function StartScreen(){
    const {setAppConfig} = useAppConfigContext()
    const [workspacePath, setWorkspacePath] = useState("");

    async function selectDirectory(){
        const directory = await open({
            multiple: false,
            directory: true,
        });

        setWorkspacePath(directory || "");
        await update_config("workspace_path", directory)
    }

    async function update_config(key: string, value: unknown){
        setAppConfig(await invoke('save_to_store', {key: key, value: value}));
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