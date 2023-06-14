/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// IssueTypeDetails : Details about an issue type.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IssueTypeDetails {
    /// The ID of the issue type's avatar.
    #[serde(rename = "avatarId", skip_serializing_if = "Option::is_none")]
    pub avatar_id: Option<i64>,
    /// The description of the issue type.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Unique ID for next-gen projects.
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    /// Hierarchy level of the issue type.
    #[serde(rename = "hierarchyLevel", skip_serializing_if = "Option::is_none")]
    pub hierarchy_level: Option<i32>,
    /// The URL of the issue type's avatar.
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// The ID of the issue type.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the issue type.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Box<crate::models::IssueTypeDetailsScope>>,
    /// The URL of these issue type details.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// Whether this issue type is used to create subtasks.
    #[serde(rename = "subtask", skip_serializing_if = "Option::is_none")]
    pub subtask: Option<bool>,
}

impl IssueTypeDetails {
    /// Details about an issue type.
    pub fn new() -> IssueTypeDetails {
        IssueTypeDetails {
            avatar_id: None,
            description: None,
            entity_id: None,
            hierarchy_level: None,
            icon_url: None,
            id: None,
            name: None,
            scope: None,
            param_self: None,
            subtask: None,
        }
    }
}
