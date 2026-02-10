# ListModels200ResponseModelsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**label** | **String** |  | 
**family** | **String** |  | 
**modality** | **Vec<Modality>** |  (enum: text, vision, audio, multimodal) | 
**capabilities** | [**models::ListModels200ResponseModelsInnerCapabilities**](ListModels200ResponseModelsInnerCapabilities.md) |  | 
**context_window** | **f64** |  | 
**max_output_tokens** | **f64** |  | 
**latency_class** | **LatencyClass** |  (enum: low, medium, high) | 
**tier** | **Tier** |  (enum: fast, standard, pro) | 
**aliases** | Option<**Vec<String>**> |  | [optional]
**default** | Option<**bool**> |  | [optional]
**deprecated_at** | Option<**String**> |  | [optional]
**policy** | Option<[**models::ListModels200ResponseModelsInnerPolicy**](ListModels200ResponseModelsInnerPolicy.md)> |  | [optional]
**metadata** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


