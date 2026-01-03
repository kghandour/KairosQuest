import { open } from '@tauri-apps/plugin-dialog';
import React, { useState } from 'react';

function StartScreen(){
    const [workspacePath, setWorkspacePath] = useState("");

    async function selectDirectory(){
        const directory = await open({
            multiple: false,
            directory: true,
        });

        setWorkspacePath(directory || "");
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