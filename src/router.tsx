import { invoke } from "@tauri-apps/api/core";
import React, { Component } from "react";
import { Route, Routes, redirect } from "react-router-dom";
import StartScreen from "./features/start_screen/StartScreen";
import Tasks from "./features/tasks/Tasks";

function AppRouter () {
    // Check if first run using config
    
    return (
        <div>
          <Routes>
              <Route path="/start_screen" element={<StartScreen />}></Route>
              <Route path="/" element={<Tasks />}></Route>
          </Routes>
        </div>
    )
}
 
export default AppRouter;