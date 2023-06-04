# ProjectRole

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actors** | Option<[**Vec<crate::models::RoleActor>**](RoleActor.md)> | The list of users who act in this role. | [optional][readonly]
**admin** | Option<**bool**> | Whether this role is the admin role for the project. | [optional][readonly]
**current_user_role** | Option<**bool**> | Whether the calling user is part of this role. | [optional]
**default** | Option<**bool**> | Whether this role is the default role for the project | [optional][readonly]
**description** | Option<**String**> | The description of the project role. | [optional][readonly]
**id** | Option<**i64**> | The ID of the project role. | [optional][readonly]
**name** | Option<**String**> | The name of the project role. | [optional]
**role_configurable** | Option<**bool**> | Whether the roles are configurable for this project. | [optional][readonly]
**scope** | Option<[**crate::models::ProjectRoleScope**](ProjectRole_scope.md)> |  | [optional]
**param_self** | Option<**String**> | The URL the project role details. | [optional][readonly]
**translated_name** | Option<**String**> | The translated name of the project role. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


