import "react";

import AboutMe from "./pages/AboutMe";
import BlogHub from "./pages/BlogHub";
import Projects from "./pages/Projects";

import { BrowserRouter, Route, Routes } from "react-router-dom";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={ <AboutMe /> }>
          <Route index element={ <div>home</div> } />
          <Route path="blogs" element={ <BlogHub /> } />
          <Route path="projects" element={ <Projects/> } />
          <Route path="*" element={ <div>not found</div> } />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
