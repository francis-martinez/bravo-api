// src/client.ts
import { build_request, WasmStreamParser } from "bravo-wasm";
var BravoClient = class {
  config;
  constructor(config) {
    this.config = config;
  }
  async *streamMessage(input) {
    const parser = new WasmStreamParser();
    const token = await this.config.getToken();
    console.log("token:", token);

    const payload = build_request(
      input.conversationId ?? null,
      input.text,
      input.module,
    );
    console.log("Payload from Rust:", payload);
    const response = await fetch(`${this.config.baseUrl}/api/chat`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
        "X-Tenant-ID": this.config.tenantId,
      },
      body: JSON.stringify(payload),
    });

    console.log("Response status:", response.status);

    if (!response.body) {
      throw new Error("Streaming not supported");
    }
    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    while (true) {
      const { value, done } = await reader.read();
      if (done) break;
      const text = decoder.decode(value);
      const events = parser.push(text);
      for (const event of events) {
        if (event.type === "text-delta") {
          yield { text: event.delta };
        }
        if (event.type === "done") {
          return;
        }
      }
    }
  }
};
export { BravoClient };
