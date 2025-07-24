import { useParams } from "react-router-dom";

export default function Blog() {
  const { title } = useParams();

  return <div>blog {title}</div>;
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
      return <p>{lines.join(" ")}</p>;
    }
    default:
      return <p>ERROR: unsupported HTML type {htmlData.type}</p>;
  }
};
