import "react";

import "./App.css";

import NavBar from "./NavBar";

import AboutMe from "./pages/AboutMe";
import BlogHub from "./pages/BlogHub";
import Projects from "./pages/Projects";

import { BrowserRouter, Route, Routes } from "react-router-dom";
import LanguageHub from "./pages/LanguageHub";
import Blog from "./pages/Blog";

function App() {
  return (
    <div className="bg-red-50 ml-[10%] mr-[10%]">
      <BrowserRouter>
        <NavBar />
        <Routes>
          <Route path="/">
            <Route index element={<AboutMe />} />
            <Route path="projects" element={<Projects />} />
            <Route path="blogs">
              <Route index element={<BlogHub />} />
              <Route path=":lang">
                <Route index element={<LanguageHub />} />
                <Route path=":title" element={<Blog />} />
              </Route>
            </Route>
            <Route path="*" element={<div>not found :&#40;</div>} />
          </Route>
        </Routes>
      </BrowserRouter>
    </div>
  );
}

export default App;
