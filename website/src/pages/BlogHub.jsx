import { useState } from "react";

export default function BlogHub() {
  // object of blogs
  const [blogs, setBlogs] = useState(null);

  if (blogs === null) {
    return <div>loading ...</div>;
  } else {
    return <div>blogs</div>;
  }
}
