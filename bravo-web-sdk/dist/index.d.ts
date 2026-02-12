interface BravoClientConfig {
    tenantId: string;
    baseUrl: string;
    getToken: () => Promise<string>;
}
interface StreamMessageInput {
    conversationId?: string;
    text: string;
    module: string;
}
declare class BravoClient {
    private config;
    private parser;
    constructor(config: BravoClientConfig);
    streamMessage(input: StreamMessageInput): AsyncGenerator<{
        text: any;
    }, void, unknown>;
}

export { BravoClient, type BravoClientConfig, type StreamMessageInput };
