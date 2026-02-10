# PostChatRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  | 
**message** | [**models::BravoChatRequestMessage**](BravoChatRequestMessage.md) |  | 
**selected_chat_model** | Option<**SelectedChatModel**> |  (enum: bravo-fast, bravo, bravo-pro) | [optional][default to BravoFast]
**selected_visibility_type** | **SelectedVisibilityType** |  (enum: public, private) | 
**selected_module** | Option<**SelectedModule**> |  (enum: spending, saving, debt, retirement, planning_insurance, planning_estate) | [optional]
**user_name** | Option<**String**> |  | [optional]
**prompt_mode** | Option<**PromptMode**> |  (enum: standard, omniprompt) | [optional]
**system_prompt_override** | Option<**String**> |  | [optional]
**general_prompt_selection** | Option<[**models::BravoChatRequestGeneralPromptSelection**](BravoChatRequestGeneralPromptSelection.md)> |  | [optional]
**module_prompt_selections** | Option<[**std::collections::HashMap<String, models::BravoChatRequestModulePromptSelectionsValue>**](BravoChatRequestModulePromptSelectionsValue.md)> |  | [optional]
**stream** | Option<**bool**> |  | [optional]
**response** | Option<**Response**> |  (enum: sse, json) | [optional]
**model** | **String** |  | 
**messages** | [**Vec<models::OpenAiChatRequestMessagesInner>**](OpenAIChatRequestMessagesInner.md) |  | 
**metadata** | Option<[**models::OpenAiChatRequestMetadata**](OpenAIChatRequestMetadata.md)> |  | [optional]
**user** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


