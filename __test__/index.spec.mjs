import test from "ava";

import { hashGlob } from "../index.js";

test("hash glob", (t) => {
  console.log(
    hashGlob(["**/*.ts", "!**/node_modules/**"], { cwd: "/workspace/tmp1" })
  );
});
