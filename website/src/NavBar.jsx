import { Link } from "react-router-dom";

export default function NavBar() {
  return (
    <nav>
      <ol className="bg-green-50 w-1/4 h-[2em] grid grid-rows-1 grid-cols-3 text-[1.5em]">
        <Tab path="/" contents="home" rightBorder />
        <Tab path="/projects" contents="projects" rightBorder />
        <Tab path="/blogs" contents="blogs" />
      </ol>
    </nav>
  );
}

/**
 * A component that centers text within a tab cell in the navigation bar.
 * @prop: path: string - path from root that the tab redirects to.
 * @prop: contents: string - what the tab displays.
 * @prop rightBorder: bool - whether to set the right border or not.
 */
function Tab({ path, contents, rightBorder = false }) {
  let classes = "list-none w-full h-full flex justify-center items-center";
  if (rightBorder) {
    classes += " border-r";
  }
  return (
    <li className={classes}>
      <Link to={path}>{contents}</Link>
    </li>
  );
}
