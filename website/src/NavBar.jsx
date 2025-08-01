import { Link } from "react-router-dom";

export default function NavBar() {
  return (
    <nav>
      <ol>
        <li>
          <Link to="/">home</Link>
        </li>
        <li>
          <Link to="/projects">projects</Link>
        </li>
        <li>
          <Link to="/blogs">blogs</Link>
        </li>
      </ol>
    </nav>
  );
}
