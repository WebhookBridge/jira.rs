/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// StatusDetailsStatusCategory : The category assigned to the status.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StatusDetailsStatusCategory {
    /// The name of the color used to represent the status category.
    #[serde(rename = "colorName", skip_serializing_if = "Option::is_none")]
    pub color_name: Option<String>,
    /// The ID of the status category.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The key of the status category.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the status category.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the status category.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
}

impl StatusDetailsStatusCategory {
    /// The category assigned to the status.
    pub fn new() -> StatusDetailsStatusCategory {
        StatusDetailsStatusCategory {
            color_name: None,
            id: None,
            key: None,
            name: None,
            param_self: None,
        }
    }
}
