import { useParams } from "react-router-dom";
import { URLtoBlog } from "../blogToURL";
import { getBlog } from "../unpack";
import parseOneLine from "../parser";

export default function Blog() {
  let { lang, title } = useParams();
  title = URLtoBlog(title);

  return (
    <>
      <title>{title}</title>
      <header className="text-[2.5em]">{title}</header>
      {getBlog(lang, title).map((html) => genHTML(html))}
    </>
  );
}

const genHTML = (htmlData) => {
  switch (htmlData.type) {
    case "Header": {
      const level = htmlData.level;
      const content = htmlData.content;
      switch (level) {
        case 1:
          return <h1 className="text-[1.5em]">{content}</h1>;
        case 2:
          return <h2 className="text-[1.25em]">{content}</h2>;
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
    case "Paragraph": {
      const lines = htmlData.lines;
      return (
        <div>
          {lines.map((line) => (
            <p>{parseOneLine(line)}</p>
          ))}
        </div>
      );
    }
    default:
      return <p>ERROR: unsupported HTML type {htmlData.type}</p>;
  }
};
