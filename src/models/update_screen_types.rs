/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateScreenTypes : The IDs of the screens for the screen types of the screen scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateScreenTypes {
    /// The ID of the create screen. To remove the screen association, pass a null.
    #[serde(rename = "create", skip_serializing_if = "Option::is_none")]
    pub create: Option<String>,
    /// The ID of the default screen. When specified, must include a screen ID as a default screen is required.
    #[serde(rename = "default", skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    /// The ID of the edit screen. To remove the screen association, pass a null.
    #[serde(rename = "edit", skip_serializing_if = "Option::is_none")]
    pub edit: Option<String>,
    /// The ID of the view screen. To remove the screen association, pass a null.
    #[serde(rename = "view", skip_serializing_if = "Option::is_none")]
    pub view: Option<String>,
}

impl UpdateScreenTypes {
    /// The IDs of the screens for the screen types of the screen scheme.
    pub fn new() -> UpdateScreenTypes {
        UpdateScreenTypes {
            create: None,
            default: None,
            edit: None,
            view: None,
        }
    }
}
