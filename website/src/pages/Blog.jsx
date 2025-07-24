import { useParams } from "react-router-dom";

export default function Blog() {
  const { title } = useParams();

  return <div>blog {title}</div>;
}
