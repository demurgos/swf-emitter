import stream, { WritableByteStream } from "@open-flash/stream";
import { Uint8 } from "semantic-types";
import * as ast from "swf-types";
import { CompressionMethod } from "swf-types";
import { emitSwf as emitSwfStream } from "./emitters/movie.js";
import { emitTag as emitTagStream } from "./emitters/tags.js";

export { ast };

/**
 * Emits an SWF movie as a byte array.
 *
 * @param value SWF movie to emit.
 * @param compressionMethod Compression method for the SWF payload.
 * @returns Corresponding byte array.
 */
export function emitSwf(
  value: ast.Movie,
  compressionMethod: CompressionMethod = CompressionMethod.None,
): Uint8Array {
  const s: WritableByteStream = new stream.WritableStream();
  emitSwfStream(s, value, compressionMethod);
  return s.getBytes();
}

/**
 * Emits an SWF tag as a byte array.
 *
 * @param value SWF tag to emit.
 * @param swfVersion SWF version to use for tags sensible to the version.
 * @returns Corresponding byte array.
 */
export function emitTag(
  value: ast.Tag,
  swfVersion: Uint8,
): Uint8Array {
  const s: WritableByteStream = new stream.WritableStream();
  emitTagStream(s, value, swfVersion);
  return s.getBytes();
}
