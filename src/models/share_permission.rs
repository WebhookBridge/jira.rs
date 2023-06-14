/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SharePermission : Details of a share permission for the filter.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SharePermission {
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<crate::models::SharePermissionGroup>>,
    /// The unique identifier of the share permission.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "project", skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<crate::models::SharePermissionProject>>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Box<crate::models::SharePermissionRole>>,
    /// The type of share permission:   *  `user` Shared with a user.  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request.
    #[serde(rename = "type")]
    pub r#type: RHashType,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::models::SharePermissionUser>>,
}

impl SharePermission {
    /// Details of a share permission for the filter.
    pub fn new(r#type: RHashType) -> SharePermission {
        SharePermission {
            group: None,
            id: None,
            project: None,
            role: None,
            r#type,
            user: None,
        }
    }
}

/// The type of share permission:   *  `user` Shared with a user.  *  `group` Shared with a group. If set in a request, then specify `sharePermission.group` as well.  *  `project` Shared with a project. If set in a request, then specify `sharePermission.project` as well.  *  `projectRole` Share with a project role in a project. This value is not returned in responses. It is used in requests, where it needs to be specify with `projectId` and `projectRoleId`.  *  `global` Shared globally. If set in a request, no other `sharePermission` properties need to be specified.  *  `loggedin` Shared with all logged-in users. Note: This value is set in a request by specifying `authenticated` as the `type`.  *  `project-unknown` Shared with a project that the user does not have access to. Cannot be set in a request.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "projectRole")]
    ProjectRole,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "loggedin")]
    Loggedin,
    #[serde(rename = "authenticated")]
    Authenticated,
    #[serde(rename = "project-unknown")]
    ProjectUnknown,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::User
    }
}
