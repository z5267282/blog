import { useParams } from "react-router-dom";
import { URLtoBlog } from "../blogToURL";
import { getBlog } from "../unpack";

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
  // find the leftmost inline feature we have to split based on:

  while (currSubLine.length > 0) {
    const link = findLink(currSubLine);
    if (link != null) {
      const { description, url, start, end } = link;
      content.push(currSubLine.slice(0, start));
      content.push(
        <a className="text-blue-400 underline" href={url} target="_blank">
          {description}
        </a>
      );
      currSubLine = currSubLine.slice(end);
      continue;
    }

    // CODE
    const code = findCode(currSubLine);
    if (code != null) {
      const { contents, start, end } = code;
      content.push(currSubLine.slice(0, start));
      content.push(<code>{contents}</code>);
      currSubLine = currSubLine.slice(end);
      continue;
    }

    // no feature found - stop parsing
    break;
  }

  // BOLD
  // ITALICS

  // before, feature and after
  // we add before to the current list of string literals and jsx that will populate the paragraph
  // we add the parsed feature
  // we keep going, but now we consider the current line to be after

  return <p>{content}</p>;
};

/**
 * For a given line, try to find a link at the start.
 * A link looks like [desc](url).
 * If there is a URL then return its description url, and the [start, end) position in the original string of the url match in an object
 * Otherwise, return null.
 * @param {*} line - string : of the current line we are looking at.
 * @returns null | object containing the description and url of the object.
 */
const findLink = (line) => {
  const pattern = /\[([^\]+]+)\]\(([^)]+)\)/;
  const attempt = line.match(pattern);
  if (attempt === null) {
    return attempt;
  }

  return {
    description: attempt[1],
    url: attempt[2],
    start: attempt.index,
    end: attempt.index + attempt[0].length,
  };
};

/**
 * For a given line, try to find an inline code snippet at the start.
 * Inline code looks like `print()` this - two backtics with non-backtics inside.
 * If there is a URL then return its contents, and the [start, end) position in the original string of the code match in an object
 * Otherwise, return null.
 * @param {*} line - string : of the current line we are looking at.
 * @returns null | object containing the description and url of the object.
 */
const findCode = (line) => {
  const pattern = /`([^`]+)`/;
  const attempt = line.match(pattern);

  if (attempt === null) {
    return attempt;
  }

  return {
    contents: attempt[1],
    start: attempt.index,
    end: attempt.index + attempt[0].length,
  };
};
