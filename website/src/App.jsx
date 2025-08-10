import "react";

import "./App.css";

import NavBar from "./NavBar";

import AboutMe from "./pages/AboutMe";
import BlogHub from "./pages/BlogHub";
import ProjectHub from "./pages/ProjectHub";

import { BrowserRouter, Route, Routes } from "react-router-dom";
import LanguageHub from "./pages/LanguageHub";
import Blog from "./pages/Blog";

export default function App() {
  return (
    <div className="w-screen min-h-screen bg-[#fff4e2] overflow-auto">
      <BrowserRouter>
        <NavBar />
        <div className="bg-white ml-[10vw] mr-[10vw] min-h-[calc(100vh-10px)]">
          <Routes>
            <Route path="/">
              <Route index element={<AboutMe />} />
              <Route path="projects" element={<ProjectHub />} />
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
        </div>
      </BrowserRouter>
    </div>
  );
}
