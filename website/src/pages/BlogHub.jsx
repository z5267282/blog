import { Link } from "react-router-dom";

import blogs from "../blog-lang.json";

export default function BlogHub() {
  return (
    <ul>
      {blogs.map((lang) => {
        const { language } = lang;
        return (
          <li>
            <Link to={`/blogs/${language}`}>{language}</Link>
          </li>
        );
      })}
    </ul>
  );
}
