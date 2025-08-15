import { expect, test } from "vitest";

import parseOneLine from "../parser";

test("parser works with plain text", () => {
  expect(parseOneLine("The big car.")).toBe(["The big car."]);
});
