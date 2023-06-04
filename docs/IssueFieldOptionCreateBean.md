# IssueFieldOptionCreateBean

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config** | Option<[**crate::models::IssueFieldOptionConfiguration**](IssueFieldOptionConfiguration.md)> |  | [optional]
**properties** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | The properties of the option as arbitrary key-value pairs. These properties can be searched using JQL, if the extractions (see https://developer.atlassian.com/cloud/jira/platform/modules/issue-field-option-property-index/) are defined in the descriptor for the issue field module. | [optional]
**value** | **String** | The option's name, which is displayed in Jira. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

