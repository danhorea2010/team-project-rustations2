import { useEffect, useState } from "react";
import "./App.css";
import { useAppDispatch, useAppSelector } from "./main";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import { Home } from "./pages/Home";
import { Todos } from "./pages/Todos";

function App() {
  const [count, setCount] = useState(0);
  const dispatch = useAppDispatch();
  const todosState = useAppSelector((state) => state.todos);

  return (
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/todo" element={<Todos />} />
        </Routes>
      </BrowserRouter>
  );
}

export default App;
