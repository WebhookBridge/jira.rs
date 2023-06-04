# Project

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**archived** | Option<**bool**> | Whether the project is archived. | [optional][readonly]
**archived_by** | Option<[**crate::models::ProjectArchivedBy**](Project_archivedBy.md)> |  | [optional]
**archived_date** | Option<**String**> | The date when the project was archived. | [optional][readonly]
**assignee_type** | Option<**String**> | The default assignee when creating issues for this project. | [optional][readonly]
**avatar_urls** | Option<[**crate::models::ProjectAvatarUrls**](Project_avatarUrls.md)> |  | [optional]
**components** | Option<[**Vec<crate::models::ProjectComponent>**](ProjectComponent.md)> | List of the components contained in the project. | [optional][readonly]
**deleted** | Option<**bool**> | Whether the project is marked as deleted. | [optional][readonly]
**deleted_by** | Option<[**crate::models::ProjectDeletedBy**](Project_deletedBy.md)> |  | [optional]
**deleted_date** | Option<**String**> | The date when the project was marked as deleted. | [optional][readonly]
**description** | Option<**String**> | A brief description of the project. | [optional][readonly]
**email** | Option<**String**> | An email address associated with the project. | [optional]
**expand** | Option<**String**> | Expand options that include additional project details in the response. | [optional][readonly]
**favourite** | Option<**bool**> | Whether the project is selected as a favorite. | [optional]
**id** | Option<**String**> | The ID of the project. | [optional]
**insight** | Option<[**crate::models::ProjectInsight**](Project_insight.md)> |  | [optional]
**is_private** | Option<**bool**> | Whether the project is private. | [optional][readonly]
**issue_type_hierarchy** | Option<[**crate::models::ProjectIssueTypeHierarchy**](Project_issueTypeHierarchy.md)> |  | [optional]
**issue_types** | Option<[**Vec<crate::models::IssueTypeDetails>**](IssueTypeDetails.md)> | List of the issue types available in the project. | [optional][readonly]
**key** | Option<**String**> | The key of the project. | [optional][readonly]
**landing_page_info** | Option<[**crate::models::ProjectLandingPageInfo**](Project_landingPageInfo.md)> |  | [optional]
**lead** | Option<[**crate::models::ProjectLead**](Project_lead.md)> |  | [optional]
**name** | Option<**String**> | The name of the project. | [optional][readonly]
**permissions** | Option<[**crate::models::ProjectPermissions**](Project_permissions.md)> |  | [optional]
**project_category** | Option<[**crate::models::ProjectProjectCategory**](Project_projectCategory.md)> |  | [optional]
**project_type_key** | Option<**String**> | The [project type](https://confluence.atlassian.com/x/GwiiLQ#Jiraapplicationsoverview-Productfeaturesandprojecttypes) of the project. | [optional][readonly]
**properties** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Map of project properties | [optional][readonly]
**retention_till_date** | Option<**String**> | The date when the project is deleted permanently. | [optional][readonly]
**roles** | Option<**::std::collections::HashMap<String, String>**> | The name and self URL for each role defined in the project. For more information, see [Create project role](#api-rest-api-3-role-post). | [optional][readonly]
**param_self** | Option<**String**> | The URL of the project details. | [optional][readonly]
**simplified** | Option<**bool**> | Whether the project is simplified. | [optional][readonly]
**style** | Option<**String**> | The type of the project. | [optional][readonly]
**url** | Option<**String**> | A link to information about this project, such as project documentation. | [optional][readonly]
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique ID for next-gen projects. | [optional][readonly]
**versions** | Option<[**Vec<crate::models::Version>**](Version.md)> | The versions defined in the project. For more information, see [Create version](#api-rest-api-3-version-post). | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


