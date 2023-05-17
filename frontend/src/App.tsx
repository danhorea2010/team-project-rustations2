import { useEffect, useState } from "react";
import "./App.css";
import { useAppDispatch, useAppSelector } from "./main";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import { Home } from "./pages/Home";
import { Todos } from "./pages/Todos";
import { Deadlines } from "./pages/Deadlines";
import { News } from "./pages/News";
import { Agenda } from "./pages/Agenda";

function App() {
  return (
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/todo" element={<Todos />} />
          <Route path="/agenda" element={<Agenda/>} />
          <Route path="/deadlines" element={<Deadlines/>} />
          <Route path="/news" element={<News />} />
        </Routes>
      </BrowserRouter>
  );
}

export default App;
