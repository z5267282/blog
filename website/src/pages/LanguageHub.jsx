import { useParams } from "react-router-dom";

export default function LanguageHub() {
  const { lang } = useParams();

  return <div>Languages for {lang}</div>;
}
