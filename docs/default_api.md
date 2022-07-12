# default_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**api.instances.tag**](default_api.md#api.instances.tag) | **GET** /instances/{SOPInstanceUID}/{tag} | Retrieve an instance field.
****](default_api.md#) | **GET** /instances | Query for instances.
****](default_api.md#) | **GET** /instances/{SOPInstanceUID}/frames/{uid} | Query for a series in a study.
****](default_api.md#) | **GET** /instances/{SOPInstanceUID} | Retrieve a instance.
****](default_api.md#) | **GET** /instances/{SOPInstanceUID}/rendered | Render an instance.
****](default_api.md#) | **GET** /instances/{SOPInstanceUID}/thumbnail | Render a thumbnail.
****](default_api.md#) | **GET** /series | Query for series.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID} | Retrieve a series.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/instances | Query for instances in the series of a study.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/frames/{uid} | Query for a series in a study.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/instances/{SOPInstanceUID} | Query for a series in a study.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/rendered | Render an instance.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/thumbnail | Render a thumbnail.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/rendered | Render an instance.
****](default_api.md#) | **GET** /series/{SeriesInstanceUID}/thumbnail | Render a thumbnail.
****](default_api.md#) | **GET** /studies | Query for studies.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID} | Retrieve a study.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series | Query for series in a study.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID} | Query for a series in a study.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances | Query for instances in the series of a study.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/frames/{uid} | Query for a series in a study.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID} | Query for a series in a study.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/rendered | Render an instance.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/instances/{SOPInstanceUID}/thumbnail | Render a thumbnail.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/rendered | Render a series.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/series/{SeriesInstanceUID}/thumbnail | Render a thumbnail.
****](default_api.md#) | **GET** /studies/{StudyInstanceUID}/thumbnail | Render a thumbnail.


# **api.instances.tag**
> serde_json::Value api.instances.tag(sop_instance_uid, tag)
Retrieve an instance field.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **sop_instance_uid** | **String**| SOPInstanceUID | 
  **tag** | **String**| A tag's group and element (e.g. 00100010) | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner> (optional)
Query for instances.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **f64**| Return only {n} results. | 
 **offset** | **f64**| Skip {n} results. | 
 **fuzzymatching** | **bool**| Whether query should use fuzzy matching. | 
 **includefield** | **String**| Include supplied tags in result. | 

### Return type

[**Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>**](_studies__StudyInstanceUID__series__SeriesInstanceUID__instances_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (sop_instance_uid, uid)
Query for a series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **sop_instance_uid** | **String**| SOPInstanceUID | 
  **uid** | **String**| Unique Indentifier | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (sop_instance_uid)
Retrieve a instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **sop_instance_uid** | **String**| SOPInstanceUID | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (sop_instance_uid, optional)
Render an instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **sop_instance_uid** | **String**| SOPInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sop_instance_uid** | **String**| SOPInstanceUID | 
 **annotation** | [**String**](String.md)| Add burned-in demographics / procedure details | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 
 **window** | [**String**](String.md)| Center of the range of gray scale in the image | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (sop_instance_uid, optional)
Render a thumbnail.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **sop_instance_uid** | **String**| SOPInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sop_instance_uid** | **String**| SOPInstanceUID | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::SeriesGet200ResponseInner> (optional)
Query for series.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **f64**| Return only {n} results. | 
 **offset** | **f64**| Skip {n} results. | 
 **fuzzymatching** | **bool**| Whether query should use fuzzy matching. | 
 **includefield** | **String**| Include supplied tags in result. | 

### Return type

[**Vec<models::SeriesGet200ResponseInner>**](_series_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (series_instance_uid)
Retrieve a series.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner> (series_instance_uid, optional)
Query for instances in the series of a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **limit** | **f64**| Return only {n} results. | 
 **offset** | **f64**| Skip {n} results. | 
 **fuzzymatching** | **bool**| Whether query should use fuzzy matching. | 
 **includefield** | **String**| Include supplied tags in result. | 

### Return type

[**Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>**](_studies__StudyInstanceUID__series__SeriesInstanceUID__instances_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (series_instance_uid, sop_instance_uid, uid)
Query for a series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 
  **uid** | **String**| Unique Indentifier | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (series_instance_uid, sop_instance_uid)
Query for a series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (series_instance_uid, sop_instance_uid, optional)
Render an instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **sop_instance_uid** | **String**| SOPInstanceUID | 
 **annotation** | [**String**](String.md)| Add burned-in demographics / procedure details | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 
 **window** | [**String**](String.md)| Center of the range of gray scale in the image | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (series_instance_uid, sop_instance_uid, optional)
Render a thumbnail.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **sop_instance_uid** | **String**| SOPInstanceUID | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (series_instance_uid, optional)
Render an instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **annotation** | [**String**](String.md)| Add burned-in demographics / procedure details | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 
 **window** | [**String**](String.md)| Center of the range of gray scale in the image | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (series_instance_uid, optional)
Render a thumbnail.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **series_instance_uid** | **String**| SeriesInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::StudiesGet200ResponseInner> (optional)
Query for studies.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **f64**| Return only {n} results. | 
 **offset** | **f64**| Skip {n} results. | 
 **fuzzymatching** | **bool**| Whether query should use fuzzy matching. | 
 **includefield** | **String**| Include supplied tags in result. | 

### Return type

[**Vec<models::StudiesGet200ResponseInner>**](_studies_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (study_instance_uid)
Retrieve a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::StudiesGet200ResponseInner> (study_instance_uid, optional)
Query for series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **limit** | **f64**| Return only {n} results. | 
 **offset** | **f64**| Skip {n} results. | 
 **fuzzymatching** | **bool**| Whether query should use fuzzy matching. | 
 **includefield** | **String**| Include supplied tags in result. | 

### Return type

[**Vec<models::StudiesGet200ResponseInner>**](_studies_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (study_instance_uid, series_instance_uid)
Query for a series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner> (study_instance_uid, series_instance_uid, optional)
Query for instances in the series of a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **limit** | **f64**| Return only {n} results. | 
 **offset** | **f64**| Skip {n} results. | 
 **fuzzymatching** | **bool**| Whether query should use fuzzy matching. | 
 **includefield** | **String**| Include supplied tags in result. | 

### Return type

[**Vec<models::StudiesStudyInstanceUidSeriesSeriesInstanceUidInstancesGet200ResponseInner>**](_studies__StudyInstanceUID__series__SeriesInstanceUID__instances_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (study_instance_uid, series_instance_uid, sop_instance_uid, uid)
Query for a series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 
  **uid** | **String**| Unique Indentifier | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> serde_json::Value (study_instance_uid, series_instance_uid, sop_instance_uid)
Query for a series in a study.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 

### Return type

[**serde_json::Value**](object.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (study_instance_uid, series_instance_uid, sop_instance_uid, optional)
Render an instance.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **sop_instance_uid** | **String**| SOPInstanceUID | 
 **annotation** | [**String**](String.md)| Add burned-in demographics / procedure details | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 
 **window** | [**String**](String.md)| Center of the range of gray scale in the image | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (study_instance_uid, series_instance_uid, sop_instance_uid, optional)
Render a thumbnail.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
  **sop_instance_uid** | **String**| SOPInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **sop_instance_uid** | **String**| SOPInstanceUID | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (study_instance_uid, series_instance_uid, optional)
Render a series.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **annotation** | [**String**](String.md)| Add burned-in demographics / procedure details | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 
 **window** | [**String**](String.md)| Center of the range of gray scale in the image | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (study_instance_uid, series_instance_uid, optional)
Render a thumbnail.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
  **series_instance_uid** | **String**| SeriesInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **series_instance_uid** | **String**| SeriesInstanceUID | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# ****
> swagger::ByteArray (study_instance_uid, optional)
Render a thumbnail.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **study_instance_uid** | **String**| StudyInstanceUID | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **study_instance_uid** | **String**| StudyInstanceUID | 
 **quality** | **f32**| Quality of image (lossy factor) | 
 **viewport** | [**i32**](i32.md)| Width and height, or crop to specific region | 

### Return type

[**swagger::ByteArray**](file.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: image/gif

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

