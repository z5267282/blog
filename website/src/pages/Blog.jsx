import { useParams } from "react-router-dom";
import { URLtoBlog } from "../blogToURL";
import { getBlog } from "../unpack";

export default function Blog() {
  let { lang, title } = useParams();
  title = URLtoBlog(title);

  return (
    <>
      <h1>blog {title}</h1>
      {getBlog(lang, title).map((html) => genHTML(html))}
    </>
  );
}

const genHTML = (htmlData) => {
  console.log(htmlData);
  switch (htmlData.type) {
    case "Header": {
      const level = htmlData.level;
      const content = htmlData.content;
      switch (level) {
        case 1:
          return <h1>{content}</h1>;
        case 2:
          return <h2>{content}</h2>;
        case 3:
          return <h3>{content}</h3>;
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
      console.log(list);
      return (
        <ol>
          {list.map((li) => (
            <li>{li}</li>
          ))}
        </ol>
      );
    }
    case "UnorderedList": {
      const list = htmlData.list;
      return (
        <ul>
          {list.map((li) => (
            <li>{li}</li>
          ))}
        </ul>
      );
    }
    case "Paragraph": {
      const lines = htmlData.lines;
      // don't assign keys for now - the index will be used
      return <>{lines.map((line) => parseAllInline(line))}</>;
    }
    default:
      return <p>ERROR: unsupported HTML type {htmlData.type}</p>;
  }
};

/**
 * Take in a line and parse all its inline HTML tags.
 * @param {*} line string of the line to parse.
 * @returns list of DOMElements of the parsed line.
 */
const parseAllInline = (line) => {
  let parsed = parseFirstInlineFeature(line);
  if (parsed === null) {
    return [<p>{line}</p>];
  }
  const domElements = [];
  while (parsed !== null) {
    const { left, right, element } = parsed;
    domElements.push(<p>{left}</p>);
    domElements.push(element);
    parsed = parseFirstInlineFeature(right);
  }
  return domElements;
};

const linkRegex = /\[([^\]+]+)\]\(([^)]+)\)/;

/**
 * Parse one line of a paragraph by trying to find the first inline element.
 * This could be: a link, bold text, italix text or inline code.
 * @param line string of the line to parse
 * @returns null | { left: string of left text, right: string of left text, element: DOMElement of parsed feature[feature as DOMElement}
 */
const parseFirstInlineFeature = (line) => {
  const link = line.match(linkRegex);
  if (link !== null) {
    const description = link[1];
    const url = link[2];
    return {
      left: line.slice(0, link.index),
      right: line.slice(link.index, link.index + link[0].length),
      element: (
        <a href={url} target="_blank">
          {description}
        </a>
      ),
    };
  }
};
