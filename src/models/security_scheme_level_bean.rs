/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecuritySchemeLevelBean {
    /// The description of the issue security scheme level.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Specifies whether the level is the default level. False by default.
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The list of level members which should be added to the issue security scheme level.
    #[serde(rename = "members", skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<crate::models::SecuritySchemeLevelMemberBean>>,
    /// The name of the issue security scheme level. Must be unique.
    #[serde(rename = "name")]
    pub name: String,
}

impl SecuritySchemeLevelBean {
    pub fn new(name: String) -> SecuritySchemeLevelBean {
        SecuritySchemeLevelBean {
            description: None,
            is_default: None,
            members: None,
            name,
        }
    }
}


