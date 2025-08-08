import Header2 from "../components/Header2";
import Header3 from "../components/Header3";
import { Link } from "react-router-dom";

export default function ProjectPreviewCard({
  url,
  title,
  dates,
  description,
  technologies,
}) {
  return (
    <Link
      to={url}
      className="block w-3/4 min-h-min bg-[#FFE1AF] rounded-lg p-5"
    >
      <div className="grid grid-cols-[70%_30%]">
        <b>
          <Header2 content={title} />
        </b>
        <b>
          <Header2 content={dates} />
        </b>
      </div>
      <b>
        <Header3 content="Description" />
      </b>
      <p>{description}</p>
      <b>
        <Header3 content="Technologies" />
      </b>
      <p>{technologies}</p>
    </Link>
  );
}
