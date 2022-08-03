const buffer = Deno.readFileSync(Deno.args[0] || "examples/hello_world.wasm");
let s = "["
for (let i = 0; i < buffer.byteLength; i++) {
  s += buffer[i] + ",";
}
s += "]\n";
Deno.core.print(s);