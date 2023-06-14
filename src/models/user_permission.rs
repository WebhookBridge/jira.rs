/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UserPermission : Details of a permission and its availability to a user.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UserPermission {
    /// Indicate whether the permission key is deprecated. Note that deprecated keys cannot be used in the `permissions parameter of Get my permissions. Deprecated keys are not returned by Get all permissions.`
    #[serde(rename = "deprecatedKey", skip_serializing_if = "Option::is_none")]
    pub deprecated_key: Option<bool>,
    /// The description of the permission.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the permission is available to the user in the queried context.
    #[serde(rename = "havePermission", skip_serializing_if = "Option::is_none")]
    pub have_permission: Option<bool>,
    /// The ID of the permission. Either `id` or `key` must be specified. Use [Get all permissions](#api-rest-api-3-permissions-get) to get the list of permissions.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the permission. Either `id` or `key` must be specified. Use [Get all permissions](#api-rest-api-3-permissions-get) to get the list of permissions.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The name of the permission.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of the permission.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl UserPermission {
    /// Details of a permission and its availability to a user.
    pub fn new() -> UserPermission {
        UserPermission {
            deprecated_key: None,
            description: None,
            have_permission: None,
            id: None,
            key: None,
            name: None,
            r#type: None,
        }
    }
}

/// The type of the permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PROJECT")]
    Project,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Global
    }
}
