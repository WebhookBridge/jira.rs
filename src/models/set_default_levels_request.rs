/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SetDefaultLevelsRequest : Details of new default levels.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDefaultLevelsRequest {
    /// List of objects with issue security scheme ID and new default level ID.
    #[serde(rename = "defaultValues")]
    pub default_values: Vec<crate::models::DefaultLevelValue>,
}

impl SetDefaultLevelsRequest {
    /// Details of new default levels.
    pub fn new(default_values: Vec<crate::models::DefaultLevelValue>) -> SetDefaultLevelsRequest {
        SetDefaultLevelsRequest {
            default_values,
        }
    }
}

