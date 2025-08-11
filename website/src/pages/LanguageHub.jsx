import { Link, useParams } from "react-router-dom";

import { getBlogTitlesForLanguage } from "../unpack";
import { blogToURL } from "../blogToURL";
import Header1 from "../components/Header1";

export default function LanguageHub() {
  const { lang } = useParams();

  const titles = Array.from(getBlogTitlesForLanguage(lang));
  titles.sort();
  
  // mobile grid: flex flex-col items-center gap-y-[20px]


  return (
    <div className="min-h-screen">
      <div className="flex justify-center items-center h-[calc(1.5em_+20px)] pt-[20px]">
        <Header1 content={`Language-Semantics for ${lang}`} />
      </div>
      <ul className="mt-10 w-full h-auto grid grid-cols-3 gap-y-[30px]">
        {titles.map((title, i) => (
          <li className={`h-[2em] w-3/4 ${alignDesktopViewPortCell(i)}`} key={`${lang} - ${title}`}>
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

/**
 * Determine the class name for a given cell on the desktop viewport.
 * There are three columns.
 * The first should be right-aligned.
 * The middle should be center-aligned.
 * The last should be left-aligned.
 * @param number - index of the current cell.
 * @returns string - of the correct class name.
 */
function alignDesktopViewPortCell(index) {
  // it had to be justify-self center since justify-items didn't work
  switch (index % 3) {
    case 0: return "justify-self-end";
    case 1: return "justify-self-center";
    // 2
    default: return "justify-self-start";
  }
}
