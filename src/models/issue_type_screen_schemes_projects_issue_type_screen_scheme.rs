/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeScreenSchemesProjectsIssueTypeScreenScheme : Details of an issue type screen scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeScreenSchemesProjectsIssueTypeScreenScheme {
    /// The description of the issue type screen scheme.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the issue type screen scheme.
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the issue type screen scheme.
    #[serde(rename = "name")]
    pub name: String,
}

impl IssueTypeScreenSchemesProjectsIssueTypeScreenScheme {
    /// Details of an issue type screen scheme.
    pub fn new(id: String, name: String) -> IssueTypeScreenSchemesProjectsIssueTypeScreenScheme {
        IssueTypeScreenSchemesProjectsIssueTypeScreenScheme {
            description: None,
            id,
            name,
        }
    }
}
