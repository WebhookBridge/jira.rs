/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectIssueTypeMapping : The project and issue type mapping.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectIssueTypeMapping {
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the project.
    #[serde(rename = "projectId")]
    pub project_id: String,
}

impl ProjectIssueTypeMapping {
    /// The project and issue type mapping.
    pub fn new(issue_type_id: String, project_id: String) -> ProjectIssueTypeMapping {
        ProjectIssueTypeMapping {
            issue_type_id,
            project_id,
        }
    }
}
