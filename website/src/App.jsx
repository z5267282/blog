import "react";

import AboutMe from "./pages/AboutMe";
import BlogHub from "./pages/BlogHub";
import Projects from "./pages/Projects";

import { BrowserRouter, Route, Routes } from "react-router-dom";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<AboutMe />}></Route>
        <Route path="/projects" element={<Projects />}></Route>
        <Route path="/blogs" element={<BlogHub />}></Route>
        <Route path="/*" element={<div>not found :&#40;</div>}></Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
