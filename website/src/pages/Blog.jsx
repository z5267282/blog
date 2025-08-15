import { useParams } from "react-router-dom";
import { URLtoBlog } from "../blogToURL";
import { getBlog } from "../unpack";
import parseOneLine from "../parser";

export default function Blog() {
  let { lang, title } = useParams();
  title = URLtoBlog(title);

  return (
    <div className="min-y-screen">
      <header className="text-[1.5em] flex justify-center items-center h-[calc(1.5em_+20px)] w-full pt-[20px]">
        {title}
      </header>
      <div className="w-full h-auto ml-[10vw] mr-[10vw] px-10">
        {getBlog(lang, title).map((html) => genHTML(html))}
      </div>
    </div>
  );
}

const genHTML = (htmlData) => {
  switch (htmlData.type) {
    case "Header": {
      const level = htmlData.level;
      const content = htmlData.content;
      switch (level) {
        case 1:
          return <h1 className="text-[1.5em] my-[0.25em]">{content}</h1>;
        case 2:
          return <h2 className="text-[1.25em] my-[0.2em]">{content}</h2>;
        case 3:
          return <h3 className="text-[1.1em]">{content}</h3>;
        case 4:
          return <h4>{content}</h4>;
        case 5:
          return <h5>{content}</h5>;
        default:
          return <h6>{content}</h6>;
      }
    }
    case "Code": {
      const code = htmlData.code;
      return (
        <pre>
          <code>{code.join("\n")}</code>
        </pre>
      );
    }
    case "OrderedList": {
      const list = htmlData.list;
      return (
        <ol>
          {list.map((li) => (
            <li className="inline list-decimal list-inside">
              {parseOneLine(li)}
            </li>
          ))}
        </ol>
      );
    }
    case "UnorderedList": {
      const list = htmlData.list;
      return (
        <ul>
          {list.map((li) => (
            <li>{parseOneLine(li)}</li>
          ))}
        </ul>
      );
    }
    case "Table": {
      const { _, headers, rows } = htmlData;
      return (
        <table className="border-[2px] border-black">
          <thead>
            <tr>
              {headers.map((header) => (
                <th className="bg-[#e2edff] p-1" scope="col">
                  {header}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr>
                {row.map((col) => (
                  <td className="p-1">{col}</td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      );
    }
    case "Paragraph": {
      const lines = htmlData.lines;
      return (
        <div>
          {lines.map((line) => (
            <p className="wrap-break-word">{parseOneLine(line)}</p>
          ))}
        </div>
      );
    }
    default:
      return <p>ERROR: unsupported HTML type {htmlData.type}</p>;
  }
};
