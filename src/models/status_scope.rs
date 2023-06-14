/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// StatusScope : The scope of the status.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusScope {
    #[serde(
        rename = "project",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub project: Option<Option<Box<crate::models::ProjectId>>>,
    /// The scope of the status. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl StatusScope {
    /// The scope of the status.
    pub fn new(r#type: RHashType) -> StatusScope {
        StatusScope {
            project: None,
            r#type,
        }
    }
}

/// The scope of the status. `GLOBAL` for company-managed projects and `PROJECT` for team-managed projects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "GLOBAL")]
    Global,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Project
    }
}
