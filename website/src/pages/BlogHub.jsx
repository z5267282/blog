import { Link } from "react-router-dom";

import { getLanguages } from "../unpack";

export default function BlogHub() {
  return (
    <ul>
      {getLanguages().map((language) => (
        <li key={`lang-${language}`}>
          <Link to={`/blogs/${language}`}>{language}</Link>
        </li>
      ))}
    </ul>
  );
}
