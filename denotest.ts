import { ping } from "./pkg/openapi.js";

async function main() {
  const pingRes = await ping();
  console.log(pingRes);
}

await main();
