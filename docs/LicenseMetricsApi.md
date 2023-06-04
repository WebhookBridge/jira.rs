# \LicenseMetricsApi

All URIs are relative to *https://your-domain.atlassian.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_approximate_application_license_count**](LicenseMetricsApi.md#get_approximate_application_license_count) | **GET** /rest/api/3/license/approximateLicenseCount/product/{applicationKey} | Get approximate application license count
[**get_approximate_license_count**](LicenseMetricsApi.md#get_approximate_license_count) | **GET** /rest/api/3/license/approximateLicenseCount | Get approximate license count



## get_approximate_application_license_count

> crate::models::LicenseMetric get_approximate_application_license_count(application_key)
Get approximate application license count

Returns the total approximate user account for a specific `jira licence application key`. Please note this information is cached with a 7-day lifecycle and could be stale at the time of call.  #### Application Key ####  An application key represents a specific version of Jira. See \\{@link ApplicationKey\\} for details  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_key** | **String** |  | [required] |

### Return type

[**crate::models::LicenseMetric**](LicenseMetric.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_approximate_license_count

> crate::models::LicenseMetric get_approximate_license_count()
Get approximate license count

Returns the total approximate user account across all jira licenced application keys. Please note this information is cached with a 7-day lifecycle and could be stale at the time of call.  **[Permissions](#permissions) required:** *Administer Jira* [global permission](https://confluence.atlassian.com/x/x4dKLg).

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LicenseMetric**](LicenseMetric.md)

### Authorization

[OAuth2](../README.md#OAuth2), [basicAuth](../README.md#basicAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

