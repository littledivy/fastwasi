import { WASI } from "node:wasi";
import { readFile } from "node:fs/promises";

const wasi = new WASI();

const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

const wasm = await WebAssembly.compile(
  await readFile(new URL("./wasi_random.wasm", import.meta.url)),
);
const instance = await WebAssembly.instantiate(wasm, importObject);

wasi.start(instance);
