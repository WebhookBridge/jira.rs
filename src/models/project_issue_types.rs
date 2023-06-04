/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectIssueTypes : Projects and issue types where the status is used. Only available if the `usages` expand is requested.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectIssueTypes {
    /// IDs of the issue types
    #[serde(rename = "issueTypes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub issue_types: Option<Option<Vec<String>>>,
    #[serde(rename = "project", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub project: Option<Option<Box<crate::models::ProjectId>>>,
}

impl ProjectIssueTypes {
    /// Projects and issue types where the status is used. Only available if the `usages` expand is requested.
    pub fn new() -> ProjectIssueTypes {
        ProjectIssueTypes {
            issue_types: None,
            project: None,
        }
    }
}

