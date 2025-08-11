import { Link, useParams } from "react-router-dom";

import { getBlogTitlesForLanguage } from "../unpack";
import { blogToURL } from "../blogToURL";
import Header1 from "../components/Header1";

export default function LanguageHub() {
  const { lang } = useParams();

  const titles = Array.from(getBlogTitlesForLanguage(lang));
  titles.sort();

  return (
    <div className="min-h-screen">
      <div className="flex justify-center items-center h-[calc(1.5em_+20px)] pt-[20px]">
        <Header1 content={`Language-Semantics for ${lang}`} />
      </div>
      <ul className="mt-10 w-full h-auto flex flex-col items-center gap-y-[20px]">
        {titles.map((title) => (
          <li className="h-[2em] w-1/3" key={`${lang} - ${title}`}>
            <Link
              className="flex justify-center items-center h-full w-full text-[1.em] bg-[#fff4e2] rounded-lg p-5 hover:bg-[#FFE1AF]"
              to={`/blogs/${lang}/${blogToURL(title)}`}
            >
              {title}
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}
