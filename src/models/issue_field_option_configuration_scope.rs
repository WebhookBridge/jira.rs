/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueFieldOptionConfigurationScope : Defines the projects that the option is available in. If the scope is not defined, then the option is available in all projects.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueFieldOptionConfigurationScope {
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<Box<crate::models::IssueFieldOptionScopeBeanGlobal>>,
    /// DEPRECATED
    #[serde(rename = "projects", skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<i64>>,
    /// Defines the projects in which the option is available and the behavior of the option within each project. Specify one object per project. The behavior of the option in a project context overrides the behavior in the global context.
    #[serde(rename = "projects2", skip_serializing_if = "Option::is_none")]
    pub projects2: Option<Vec<crate::models::ProjectScopeBean>>,
}

impl IssueFieldOptionConfigurationScope {
    /// Defines the projects that the option is available in. If the scope is not defined, then the option is available in all projects.
    pub fn new() -> IssueFieldOptionConfigurationScope {
        IssueFieldOptionConfigurationScope {
            global: None,
            projects: None,
            projects2: None,
        }
    }
}
