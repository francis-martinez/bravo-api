import { build_request, WasmStreamParser } from "bravo-wasm";

export interface BravoClientConfig {
  tenantId: string;
  baseUrl: string;
  getToken: () => Promise<string>;
}

export interface StreamMessageInput {
  conversationId?: string;
  text: string;
  module: string;
}

export class BravoClient {
  private config: BravoClientConfig;
  private parser: WasmStreamParser;

  constructor(config: BravoClientConfig) {
    this.config = config;

    this.parser = new WasmStreamParser();
  }

  async *streamMessage(input: StreamMessageInput) {
    const token = await this.config.getToken();

    const payload = build_request(
      input.conversationId ?? null,
      input.text,
      input.module,
    );

    const response = await fetch(`${this.config.baseUrl}/api/chat`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
        "X-Tenant-ID": this.config.tenantId,
      },
      body: JSON.stringify(payload),
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
}
