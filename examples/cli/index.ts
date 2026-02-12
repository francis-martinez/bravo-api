import { WasmStreamParser } from "bravo-wasm";

async function main() {
  const parser = new WasmStreamParser();

  const fakeChunks = [
    '{"type":"text-delta","id":"0","delta":"help "}\n',
    '{"type":"text-delta","id":"0","delta":"me "}\n',
    "[DONE]\n",
  ];

  for (const chunk of fakeChunks) {
    const events = parser.push(chunk);

    console.log("Chunk:", chunk);
    console.log("Events:", events);
  }
}

main();
