import { expect, test } from "vitest";
import { render } from "vitest-browser-react";

import parseOneLine from "../parser";

test("parser works with plain text", () => {
  expect(parseOneLine("The big car.")).toStrictEqual(["The big car."]);
});

test("parser works with link in the middle", async () => {
  const parsed = parseOneLine(
    "There is info at [this link](https://www.google.com) for some more information"
  );
  expect(parsed.length).toBe(3);

  const [start, Hyperlink, end] = parsed;
  expect(start).toBe("There is info at ");
  expect(end).toBe(" for some more information");

  const { getByText } = render(Hyperlink);
  const link = getByText("this link");
  await expect.element(link).toBeInTheDocument();
  await expect.element(link).toHaveAttribute("href", "https://www.google.com");
  await expect.element(link).toHaveTextContent("this link");
});
