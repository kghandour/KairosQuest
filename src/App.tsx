import "./App.css";
import { BrowserRouter } from "react-router-dom";
import { ConfigProvider } from "./shared/contexts/ConfigContext";
import AppRouter from "./Router";

function App() {
  return (
    <BrowserRouter>
      <ConfigProvider>
        <AppRouter />
      </ConfigProvider>
    </BrowserRouter>
  );
}

export default App;
