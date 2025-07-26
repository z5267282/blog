import { Link, useParams } from "react-router-dom";

import { getBlogTitlesForLanguage } from "../unpack";
import { blogToURL } from "../blogToURL";

export default function LanguageHub() {
  const { lang } = useParams();

  return (
    <>
      <h1>blogs for {lang}</h1>
      <ul>
        {getBlogTitlesForLanguage(lang).map((title) => (
          <li key={`${lang} - ${title}`}>
            <Link to={`/blogs/${lang}/${blogToURL(title)}`}>{title}</Link>
          </li>
        ))}
      </ul>
    </>
  );
}
