/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectRoleScope : The scope of the role. Indicated for roles associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectRoleScope {
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<crate::models::ScopeProject>>,
    /// The type of scope.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl ProjectRoleScope {
    /// The scope of the role. Indicated for roles associated with [next-gen projects](https://confluence.atlassian.com/x/loMyO).
    pub fn new() -> ProjectRoleScope {
        ProjectRoleScope {
            project: None,
            r#type: None,
        }
    }
}

/// The type of scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "TEMPLATE")]
    Template,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Project
    }
}
