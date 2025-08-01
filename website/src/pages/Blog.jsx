import { useParams } from "react-router-dom";
import { URLtoBlog } from "../blogToURL";
import { getBlog } from "../unpack";
import { parsers } from "../inline-parse/parser";

export default function Blog() {
  let { lang, title } = useParams();
  title = URLtoBlog(title);

  return (
    <>
      <title>{title}</title>
      <header>blog {title}</header>
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
      return <div>{lines.map((line) => parseOneLine(line))}</div>;
    }
    default:
      return <p>ERROR: unsupported HTML type {htmlData.type}</p>;
  }
};

/**
 * Take in one line and parse it into a paragraph.
 * This paragraph can contained nested inline elements like links, code and bold text.
 * @param {*} lineContents - string : of the current line.
 * @returns A <p> tag with the line contents
 */
const parseOneLine = (lineContents) => {
  const content = [];
  let currSubLine = lineContents;

  while (currSubLine.length > 0) {
    const first = findLeftMostFeature(currSubLine);
    if (first === null) {
      break;
    }

    const { jsx, start, end } = first;
    content.push(currSubLine.slice(0, start));
    content.push(jsx);
    currSubLine = currSubLine.slice(end);
  }

  return <p>{content}</p>;
};

/**
 * Find the left most feature of a line.
 * @param {*} line : string - the current line we are looking at.
 * @returns null - if there was no feature on the line.
 * @returns object with the [start, end) position in the original line of the match and jsx of the parsed element.
 */
const findLeftMostFeature = (currSubLine) => {
  const parsedOptions = parsers.map((parser) => parser.tryParse(currSubLine));
  let earliest = null;
  for (const option of parsedOptions) {
    if (option === null) {
      continue;
    }

    if (earliest === null || option.start < earliest.start) {
      earliest = option;
    }
  }
  return earliest;
};
