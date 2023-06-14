/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeSchemeMapping : Issue type scheme item.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeSchemeMapping {
    /// The ID of the issue type.
    #[serde(rename = "issueTypeId")]
    pub issue_type_id: String,
    /// The ID of the issue type scheme.
    #[serde(rename = "issueTypeSchemeId")]
    pub issue_type_scheme_id: String,
}

impl IssueTypeSchemeMapping {
    /// Issue type scheme item.
    pub fn new(issue_type_id: String, issue_type_scheme_id: String) -> IssueTypeSchemeMapping {
        IssueTypeSchemeMapping {
            issue_type_id,
            issue_type_scheme_id,
        }
    }
}
