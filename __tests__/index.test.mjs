import "@jest/globals";
import path from "path";
import { fileURLToPath } from "url";
import { hashGlobParallel, hashGlob } from "../index.js";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

describe("hash glob parallel", () => {
  it("should calculate the hash in parallel consistently", () => {
    const map = hashGlobParallel(
      ["a.*"],
      {
        cwd: path.join(__dirname, "fixtures"),
        concurrency: 200,
      }
    );

    expect(map).toMatchInlineSnapshot(`
{
  "a.json": 11810798349410098695n,
  "a.png": 4573747350076585403n,
  "a.txt": 13554666155361377856n,
}
`);
  });

  it("should calculate the hash in serial consistently", () => {
    const map = hashGlob(
      ["a.*"],
      {
        cwd: path.join(__dirname, "fixtures"),
        concurrency: 200,
      }
    );

    expect(map).toMatchInlineSnapshot(`
{
  "a.json": 11810798349410098695n,
  "a.png": 4573747350076585403n,
  "a.txt": 13554666155361377856n,
}
`);
  });

  it("should calculate the hash of both positive and negative match globs", () => {
    const map = hashGlob(
      ["*.*", "!b.*"],
      {
        cwd: path.join(__dirname, "fixtures"),
        concurrency: 200,
      }
    );

    expect(map).toMatchInlineSnapshot(`
{
  ".hiddenfile": 9323281055126355666n,
  "a.json": 11810798349410098695n,
  "a.png": 4573747350076585403n,
  "a.txt": 13554666155361377856n,
}
`);
  });

  it("should calculate the hash of dot files", () => {
    const map = hashGlob(
      [],
      {
        cwd: path.join(__dirname, "fixtures"),
        concurrency: 200,
      }
    );

    expect(map).toMatchInlineSnapshot(`
{
  ".hiddenfile": 9323281055126355666n,
  "a.json": 11810798349410098695n,
  "a.png": 4573747350076585403n,
  "a.txt": 13554666155361377856n,
  "b.txt": 11083092647103983954n,
}
`);
  });
});
