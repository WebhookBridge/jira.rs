/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UpdateScreenSchemeDetails : Details of a screen scheme.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UpdateScreenSchemeDetails {
    /// The description of the screen scheme. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the screen scheme. The name must be unique. The maximum length is 255 characters.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "screens", skip_serializing_if = "Option::is_none")]
    pub screens: Option<Box<crate::models::UpdateScreenSchemeDetailsScreens>>,
}

impl UpdateScreenSchemeDetails {
    /// Details of a screen scheme.
    pub fn new() -> UpdateScreenSchemeDetails {
        UpdateScreenSchemeDetails {
            description: None,
            name: None,
            screens: None,
        }
    }
}


