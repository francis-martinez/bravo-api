use uuid::Uuid;

use crate::models::openapi::{
    bravo_chat_request::{Response, SelectedChatModel, SelectedModule, SelectedVisibilityType},
    bravo_chat_request_message::Role,
    bravo_chat_request_message_parts_inner::Type,
    BravoChatRequest, BravoChatRequestMessage, BravoChatRequestMessagePartsInner,
};

pub struct SendMessageInput {
    pub conversation_id: Option<Uuid>,
    pub text: String,
    pub module: SelectedModule,
    pub model: Option<SelectedChatModel>,
    pub visibility: Option<SelectedVisibilityType>,
}

pub fn build_send_message(input: SendMessageInput) -> BravoChatRequest {
    let conversation_id = input.conversation_id.unwrap_or_else(Uuid::new_v4);

    let message = BravoChatRequestMessage {
        id: Uuid::new_v4(),
        parts: vec![BravoChatRequestMessagePartsInner {
            text: input.text,
            r#type: Type::Text,
        }],
        role: Role::User,
    };

    let mut request = BravoChatRequest::new(
        conversation_id,
        message,
        input.visibility.unwrap_or(SelectedVisibilityType::Private),
    );

    request.selected_chat_model = Some(input.model.unwrap_or(SelectedChatModel::Bravo));

    request.selected_module = Some(input.module);

    request.stream = Some(true);

    request.response = Some(Response::Sse);

    request
}
