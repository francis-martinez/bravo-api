use uuid::Uuid;
use wasm_bindgen::prelude::*;

use bravo_core::{SendMessageInput, build_send_message};

use bravo_core::models::openapi::bravo_chat_request::{
    SelectedChatModel, SelectedModule, SelectedVisibilityType,
};

#[wasm_bindgen]
pub fn build_request(conversation_id: Option<String>, text: String, module: String) -> JsValue {
    let conversation_uuid = conversation_id.and_then(|id| Uuid::parse_str(&id).ok());

    let module_enum = match module.as_str() {
        "spending" => SelectedModule::Spending,
        "saving" => SelectedModule::Saving,
        "debt" => SelectedModule::Debt,
        "retirement" => SelectedModule::Retirement,
        "planning_insurance" => SelectedModule::PlanningInsurance,
        "planning_estate" => SelectedModule::PlanningEstate,
        _ => SelectedModule::Spending,
    };

    let input = SendMessageInput {
        conversation_id: conversation_uuid,
        text,
        module: module_enum,
        model: Some(SelectedChatModel::Bravo),
        visibility: Some(SelectedVisibilityType::Private),
    };

    let request = build_send_message(input);

    serde_wasm_bindgen::to_value(&request).unwrap()
}
