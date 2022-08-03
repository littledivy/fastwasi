// Note: this is identical to deno_std.js and does not use Node's WASI implementation.
// TODO: Node's WASI implementation may be faster than this.

import { readFile } from "node:fs/promises";
import { webcrypto as crypto } from "node:crypto";

function benchSync(name, n, innerLoop) {
  const t1 = Date.now();
  for (let i = 0; i < n; i++) {
    innerLoop(i);
  }
  const t2 = Date.now();
  console.log(benchStats(name, n, t1, t2));
}

function benchStats(name, n, t1, t2) {
  const dt = (t2 - t1) / 1e3;
  const r = n / dt;
  const ns = Math.floor(dt / n * 1e9);
  return `${name}:${" ".repeat(30 - name.length)}\t` +
    `n = ${n}, dt = ${dt.toFixed(3)}s, r = ${r.toFixed(0)}/s, t = ${ns}ns/op`;
}
const module = await WebAssembly.compile(
  await readFile("examples/hello_world.wasm"),
);
let memory;
const instance = await WebAssembly.instantiate(module, {
  env: {
    "random_get": function (
      bufferOffset,
      bufferLength,
    ) {
      const buffer = new Uint8Array(
        memory.buffer,
        bufferOffset,
        bufferLength,
      );
      crypto.getRandomValues(buffer);

      return 0; // ERRNO_SUCCESS
    },
  },
});
const { main } = instance.exports;
memory = instance.exports.memory;
if (main() !== 0) {
  throw new Error("main() failed");
}

while (true) benchSync(`main (${memory.buffer.byteLength} bytes)`, 1e3, main);
