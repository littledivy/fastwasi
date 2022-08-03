import Context from "https://deno.land/std@0.150.0/wasi/snapshot_preview1.ts";

const context = new Context();

const binary = await Deno.readFile(
  new URL("./wasi_random.wasm", import.meta.url),
);
const module = await WebAssembly.compile(binary);
const instance = await WebAssembly.instantiate(module, {
  "wasi_snapshot_preview1": context.exports,
});

context.start(instance);
