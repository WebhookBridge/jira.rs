/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ScreenWithTabTab : The tab for the screen.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenWithTabTab {
    /// The ID of the screen tab.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the screen tab. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
}

impl ScreenWithTabTab {
    /// The tab for the screen.
    pub fn new(name: String) -> ScreenWithTabTab {
        ScreenWithTabTab { id: None, name }
    }
}
