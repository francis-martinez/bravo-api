use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TextDeltaEvent {
    pub r#type: String,
    pub id: String,
    pub delta: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamEvent {
    #[serde(rename = "text-delta")]
    TextDelta { id: String, delta: String },

    #[serde(rename = "done")]
    Done,
}

#[wasm_bindgen]
pub struct StreamParser {
    buffer: String,
}

#[wasm_bindgen]
impl StreamParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> StreamParser {
        StreamParser {
            buffer: String::new(),
        }
    }

    pub fn push(&mut self, chunk: &str) -> JsValue {
        let events = self.push_internal(chunk);
        serde_wasm_bindgen::to_value(&events).unwrap()
    }
}

impl StreamParser {
    fn push_internal(&mut self, chunk: &str) -> Vec<StreamEvent> {
        self.buffer.push_str(chunk);

        let mut events = Vec::new();

        loop {
            if let Some(pos) = self.buffer.find('\n') {
                let line = self.buffer[..pos].trim().to_string();
                self.buffer = self.buffer[pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if line == "[DONE]" {
                    events.push(StreamEvent::Done);
                    continue;
                }

                if let Ok(parsed) = serde_json::from_str::<TextDeltaEvent>(&line) {
                    events.push(StreamEvent::TextDelta {
                        id: parsed.id,
                        delta: parsed.delta,
                    });
                }
            } else {
                break;
            }
        }

        events
    }
}
