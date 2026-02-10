# \DefaultApi

All URIs are relative to *https://0.0.0.0:3000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_chat**](DefaultApi.md#delete_chat) | **DELETE** /api/v1/chat | Delete chat
[**get_model**](DefaultApi.md#get_model) | **GET** /api/v1/models/{id} | Get a model definition
[**get_ping**](DefaultApi.md#get_ping) | **GET** /api/v1/ping | Health probe
[**get_user_context**](DefaultApi.md#get_user_context) | **GET** /api/v1/user/context | Get user context and persona
[**list_chat_messages**](DefaultApi.md#list_chat_messages) | **GET** /api/v1/chats/{id}/messages | List messages for a chat
[**list_chats**](DefaultApi.md#list_chats) | **GET** /api/v1/chats | List chats
[**list_models**](DefaultApi.md#list_models) | **GET** /api/v1/models | List tenant-allowed models
[**post_chat**](DefaultApi.md#post_chat) | **POST** /api/v1/chat | Create/continue chat (SSE or JSON)
[**upload_file**](DefaultApi.md#upload_file) | **POST** /api/v1/files/upload | Upload a file



## delete_chat

> models::DeleteChatResponse delete_chat(id)
Delete chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteChatResponse**](DeleteChatResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_model

> models::GetModel200Response get_model(id)
Get a model definition

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetModel200Response**](getModel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ping

> models::PingResponse get_ping()
Health probe

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PingResponse**](PingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_context

> models::UserContextResponse get_user_context()
Get user context and persona

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UserContextResponse**](UserContextResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_chat_messages

> models::ChatMessagesResponse list_chat_messages(id, cursor, limit, direction)
List messages for a chat

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**cursor** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |
**direction** | Option<**String**> |  |  |

### Return type

[**models::ChatMessagesResponse**](ChatMessagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_chats

> models::ChatsListResponse list_chats(cursor, limit, visibility, module, mode)
List chats

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |
**visibility** | Option<**String**> |  |  |
**module** | Option<**String**> |  |  |
**mode** | Option<**String**> |  |  |

### Return type

[**models::ChatsListResponse**](ChatsListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_models

> models::ListModels200Response list_models(family, modality, capability, tier)
List tenant-allowed models

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**family** | Option<**String**> |  |  |
**modality** | Option<**String**> |  |  |
**capability** | Option<**String**> |  |  |
**tier** | Option<**String**> |  |  |

### Return type

[**models::ListModels200Response**](listModels_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_chat

> models::ChatJsonResponse post_chat(post_chat_request)
Create/continue chat (SSE or JSON)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**post_chat_request** | [**PostChatRequest**](PostChatRequest.md) |  | [required] |

### Return type

[**models::ChatJsonResponse**](ChatJSONResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/event-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> models::FileUploadResponse upload_file(file, purpose)
Upload a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | **std::path::PathBuf** |  | [required] |
**purpose** | Option<**String**> |  |  |

### Return type

[**models::FileUploadResponse**](FileUploadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

