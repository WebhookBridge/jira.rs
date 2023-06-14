/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RoleActor : Details about a user assigned to a project role.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RoleActor {
    #[serde(rename = "actorGroup", skip_serializing_if = "Option::is_none")]
    pub actor_group: Option<Box<crate::models::ProjectRoleGroup>>,
    #[serde(rename = "actorUser", skip_serializing_if = "Option::is_none")]
    pub actor_user: Option<Box<crate::models::ProjectRoleUser>>,
    /// The avatar of the role actor.
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// The display name of the role actor. For users, depending on the user’s privacy setting, this may return an alternative value for the user's name.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The ID of the role actor.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The type of role actor.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<RHashType>,
}

impl RoleActor {
    /// Details about a user assigned to a project role.
    pub fn new() -> RoleActor {
        RoleActor {
            actor_group: None,
            actor_user: None,
            avatar_url: None,
            display_name: None,
            id: None,
            name: None,
            r#type: None,
        }
    }
}

/// The type of role actor.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "atlassian-group-role-actor")]
    GroupRoleActor,
    #[serde(rename = "atlassian-user-role-actor")]
    UserRoleActor,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::GroupRoleActor
    }
}
