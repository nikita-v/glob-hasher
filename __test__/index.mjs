import { hashGlob, hashGlobParallel } from "../index.js";

const map = hashGlobParallel(["packages/**/*.ts", "packages/**/*.tsx", "!**/node_modules/**"], {
  cwd: "/Users/ken/workspace/tmp1",
})

console.log(Object.keys(map).length);
