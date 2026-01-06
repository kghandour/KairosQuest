import { Link } from "react-router-dom";
import { invoke } from '@tauri-apps/api/core';

function Tasks(){
    async function create_file(){
        console.log(await invoke("create_file", {name: "test.json", content: "{value: 1}"}))
    }

    return (
        <div>
            <h1>Tasks here</h1>
            <h2>Initial screen test <Link to="/start_screen">here</Link></h2>
            <button onClick={create_file}>Create new file</button>
        </div>
    )
}

export default Tasks;