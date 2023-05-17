import { useEffect, useState } from "react";
import "./App.css";
import { useAppDispatch, useAppSelector } from "./main";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import { Home } from "./pages/Home";
import { Todos } from "./pages/Todos";

function App() {
  return (
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/todo" element={<Todos />} />
          <Route path="/agenda" element={<Todos />} />
          <Route path="/deadlines" element={<Todos />} />
          <Route path="/news" element={<Todos />} />
        </Routes>
      </BrowserRouter>
  );
}

export default App;
