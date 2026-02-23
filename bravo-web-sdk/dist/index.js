// src/client.ts
import { build_request, StreamParser } from "bravo-wasm";
var BravoClient = class {
  config;
  parser;
  constructor(config) {
    this.config = config;
    this.parser = new StreamParser();
  }
  async *streamMessage(input) {
    const token = await this.config.getToken();
    const payload = build_request(
      input.conversationId ?? null,
      input.text,
      input.module
    );
    const response = await fetch(`${this.config.baseUrl}/api/chat`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
        "X-Tenant-ID": this.config.tenantId
      },
      body: JSON.stringify(payload)
    });
    if (!response.body) {
      throw new Error("Streaming not supported");
    }
    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    while (true) {
      const { value, done } = await reader.read();
      if (done) break;
      const text = decoder.decode(value);
      const events = this.parser.push(text);
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
export {
  BravoClient
};
