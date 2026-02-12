// import { WasmStreamParser } from "bravo-wasm";

// async function main() {
//   const parser = new WasmStreamParser();

//   const fakeChunks = [
//     '{"type":"text-delta","id":"0","delta":"help "}\n',
//     '{"type":"text-delta","id":"0","delta":"me "}\n',
//     "[DONE]\n",
//   ];

//   for (const chunk of fakeChunks) {
//     const events = parser.push(chunk);

//     console.log("Chunk:", chunk);
//     console.log("Events:", events);
//   }
// }

// main();

import { BravoClient } from "bravo-web-sdk";

async function run() {
  const client = new BravoClient({
    tenantId: "dev",
    baseUrl: "http://localhost:3000",
    getToken: async () => "fake-token",
  });

  for await (const chunk of client.streamMessage({
    text: "Help me analyze my spending",
    module: "spending",
  })) {
    process.stdout.write(chunk.text ?? "");
  }
}

run();
