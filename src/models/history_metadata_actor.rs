/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// HistoryMetadataActor : Details of the user whose action created the history record.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoryMetadataActor {
    /// The URL to an avatar for the user or system associated with a history record.
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the user or system associated with a history record.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The key of the display name of the user or system associated with a history record.
    #[serde(rename = "displayNameKey", skip_serializing_if = "Option::is_none")]
    pub display_name_key: Option<String>,
    /// The ID of the user or system associated with a history record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The type of the user or system associated with a history record.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The URL of the user or system associated with a history record.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl HistoryMetadataActor {
    /// Details of the user whose action created the history record.
    pub fn new() -> HistoryMetadataActor {
        HistoryMetadataActor {
            avatar_url: None,
            display_name: None,
            display_name_key: None,
            id: None,
            r#type: None,
            url: None,
        }
    }
}
