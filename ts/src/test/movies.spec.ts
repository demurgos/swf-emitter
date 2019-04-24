import chai from "chai";
import fs from "fs";
import { JsonReader } from "kryo/readers/json";
import { JsonValueWriter } from "kryo/writers/json-value";
import sysPath from "path";
import { movieFromBytes } from "swf-parser";
import { CompressionMethod } from "swf-tree";
import { $Movie, Movie } from "swf-tree/movie";
import { movieToBytes } from "../lib";
import meta from "./meta.js";
import { readTextFile } from "./utils";

const PROJECT_ROOT: string = sysPath.join(meta.dirname, "..", "..", "..");
const MOVIE_SAMPLES_ROOT: string = sysPath.join(PROJECT_ROOT, "..", "tests", "movies");

const JSON_READER: JsonReader = new JsonReader();
const JSON_VALUE_WRITER: JsonValueWriter = new JsonValueWriter();
// `BLACKLIST` can be used to forcefully skip some tests.
const BLACKLIST: ReadonlySet<string> = new Set([]);
// `WHITELIST` can be used to only enable a few tests.
const WHITELIST: ReadonlySet<string> = new Set([
  // "hello-world",
]);

describe("movies", function () {
  this.timeout(300000); // The timeout is this high due to CI being extremely slow

  for (const sample of getSamples()) {
    it(sample.name, async function () {
      const valueJson: string = await readTextFile(sample.astPath);
      const value: Movie = $Movie.read(JSON_READER, valueJson);

      const actualBytes: Uint8Array = movieToBytes(value, CompressionMethod.None);

      fs.writeFileSync(sysPath.join(MOVIE_SAMPLES_ROOT, sample.name, "local-main.ts.swf"), actualBytes);

      const actualMovie: Movie = movieFromBytes(actualBytes);

      try {
        chai.assert.isTrue($Movie.equals(actualMovie, value));
      } catch (err) {
        chai.assert.strictEqual(
          JSON.stringify($Movie.write(JSON_VALUE_WRITER, actualMovie), null, 2),
          valueJson,
        );
        throw err;
      }
    });
  }
});

interface Sample {
  name: string;
}

interface Sample {
  name: string;
  moviePath: string;
  astPath: string;
}

function* getSamples(): IterableIterator<Sample> {
  for (const dirEnt of fs.readdirSync(MOVIE_SAMPLES_ROOT, {withFileTypes: true})) {
    if (!dirEnt.isDirectory()) {
      continue;
    }
    const testName: string = dirEnt.name;
    const testPath: string = sysPath.join(MOVIE_SAMPLES_ROOT, testName);

    if (BLACKLIST.has(testName)) {
      continue;
    } else if (WHITELIST.size > 0 && !WHITELIST.has(testName)) {
      continue;
    }

    const moviePath: string = sysPath.join(testPath, "main.swf");
    const astPath: string = sysPath.join(testPath, "ast.json");

    yield {name: testName, moviePath, astPath};
  }
}
