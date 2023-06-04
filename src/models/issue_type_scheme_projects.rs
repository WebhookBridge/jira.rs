/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeSchemeProjects : Issue type scheme with a list of the projects that use it.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeSchemeProjects {
    #[serde(rename = "issueTypeScheme")]
    pub issue_type_scheme: Box<crate::models::IssueTypeSchemeProjectsIssueTypeScheme>,
    /// The IDs of the projects using the issue type scheme.
    #[serde(rename = "projectIds")]
    pub project_ids: Vec<String>,
}

impl IssueTypeSchemeProjects {
    /// Issue type scheme with a list of the projects that use it.
    pub fn new(issue_type_scheme: crate::models::IssueTypeSchemeProjectsIssueTypeScheme, project_ids: Vec<String>) -> IssueTypeSchemeProjects {
        IssueTypeSchemeProjects {
            issue_type_scheme: Box::new(issue_type_scheme),
            project_ids,
        }
    }
}


