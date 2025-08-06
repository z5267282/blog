import "react";

import "./App.css";

import NavBar from "./NavBar";

import AboutMe from "./pages/AboutMe";
import BlogHub from "./pages/BlogHub";
import Projects from "./pages/Projects";

import { BrowserRouter, Route, Routes } from "react-router-dom";
import LanguageHub from "./pages/LanguageHub";
import Blog from "./pages/Blog";

export default function App() {
  return (
    // the background should be as big as wide as the screen, and at least as tall as the screen where scrolling is allowed.
    <div className="w-screen min-h-screen bg-[#fff4e2] overflow-auto">
      <div className="bg-white ml-[20%] mr-[20%] min-h-screen">
        <DynamicContent />
      </div>
    </div>
  );
}

function DynamicContent() {
  return (
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
  );
}
