import blogs from "../blog-lang.json";

export default function BlogHub() {
  return (
    <div>
      {blogs.map((lang) => {
        const { language, blogs } = lang;
        return (
          <>
            <h1>{language}</h1>
            {blogs.map((blog) => {
              console.log(blog.html);
              return (
                <>
                  <h2>{blog.title}</h2>
                  <div>{blog.html.map((h) => genHTML(h))}</div>
                </>
              );
            })}
          </>
        );
      })}
    </div>
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
      return <p>{lines.join(" ")}</p>;
    }
    default:
      return <p>ERROR: unsupported HTML type {htmlData.type}</p>;
  }
};
