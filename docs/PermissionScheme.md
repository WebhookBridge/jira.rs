# PermissionScheme

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | A description for the permission scheme. | [optional]
**expand** | Option<**String**> | The expand options available for the permission scheme. | [optional][readonly]
**id** | Option<**i64**> | The ID of the permission scheme. | [optional][readonly]
**name** | **String** | The name of the permission scheme. Must be unique. | 
**permissions** | Option<[**Vec<crate::models::PermissionGrant>**](PermissionGrant.md)> | The permission scheme to create or update. See [About permission schemes and grants](../api-group-permission-schemes/#about-permission-schemes-and-grants) for more information. | [optional]
**scope** | Option<[**crate::models::PermissionSchemeScope**](PermissionScheme_scope.md)> |  | [optional]
**param_self** | Option<**String**> | The URL of the permission scheme. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


