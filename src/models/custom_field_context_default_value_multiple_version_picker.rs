/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueMultipleVersionPicker : The default value for a multiple version picker custom field.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueMultipleVersionPicker {
    #[serde(rename = "type")]
    pub r#type: String,
    /// The IDs of the default versions.
    #[serde(rename = "versionIds")]
    pub version_ids: Vec<String>,
    /// The order the pickable versions are displayed in. If not provided, the released-first order is used. Available version orders are `\"releasedFirst\"` and `\"unreleasedFirst\"`.
    #[serde(rename = "versionOrder", skip_serializing_if = "Option::is_none")]
    pub version_order: Option<String>,
}

impl CustomFieldContextDefaultValueMultipleVersionPicker {
    /// The default value for a multiple version picker custom field.
    pub fn new(
        r#type: String,
        version_ids: Vec<String>,
    ) -> CustomFieldContextDefaultValueMultipleVersionPicker {
        CustomFieldContextDefaultValueMultipleVersionPicker {
            r#type,
            version_ids,
            version_order: None,
        }
    }
}
