/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ScreenWithTab : A screen with tab details.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenWithTab {
    /// The description of the screen.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The ID of the screen.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the screen.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<crate::models::ScreenScope>>,
    #[serde(rename = "tab", skip_serializing_if = "Option::is_none")]
    pub tab: Option<Box<crate::models::ScreenWithTabTab>>,
}

impl ScreenWithTab {
    /// A screen with tab details.
    pub fn new() -> ScreenWithTab {
        ScreenWithTab {
            description: None,
            id: None,
            name: None,
            scope: None,
            tab: None,
        }
    }
}

