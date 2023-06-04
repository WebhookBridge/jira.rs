/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SharePermissionInputBean {
    /// The user account ID that the filter is shared with. For a request, specify the `accountId` property for the user.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The ID of the group, which uniquely identifies the group across all Atlassian products.For example, *952d12c3-5b5b-4d04-bb32-44d383afc4b2*. Cannot be provided with `groupname`.
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// The name of the group to share the filter with. Set `type` to `group`. Please note that the name of a group is mutable, to reliably identify a group use `groupId`.
    #[serde(rename = "groupname", skip_serializing_if = "Option::is_none")]
    pub groupname: Option<String>,
    /// The ID of the project to share the filter with. Set `type` to `project`.
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The ID of the project role to share the filter with. Set `type` to `projectRole` and the `projectId` for the project that the role is in.
    #[serde(rename = "projectRoleId", skip_serializing_if = "Option::is_none")]
    pub project_role_id: Option<String>,
    /// The rights for the share permission.
    #[serde(rename = "rights", skip_serializing_if = "Option::is_none")]
    pub rights: Option<i32>,
    /// The type of the share permission.Specify the type as follows:   *  `user` Share with a user.  *  `group` Share with a group. Specify `groupname` as well.  *  `project` Share with a project. Specify `projectId` as well.  *  `projectRole` Share with a project role in a project. Specify `projectId` and `projectRoleId` as well.  *  `global` Share globally, including anonymous users. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.  *  `authenticated` Share with all logged-in users. This shows as `loggedin` in the response. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl SharePermissionInputBean {
    pub fn new(r#type: RHashType) -> SharePermissionInputBean {
        SharePermissionInputBean {
            account_id: None,
            group_id: None,
            groupname: None,
            project_id: None,
            project_role_id: None,
            rights: None,
            r#type,
        }
    }
}

/// The type of the share permission.Specify the type as follows:   *  `user` Share with a user.  *  `group` Share with a group. Specify `groupname` as well.  *  `project` Share with a project. Specify `projectId` as well.  *  `projectRole` Share with a project role in a project. Specify `projectId` and `projectRoleId` as well.  *  `global` Share globally, including anonymous users. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.  *  `authenticated` Share with all logged-in users. This shows as `loggedin` in the response. If set, this type overrides all existing share permissions and must be deleted before any non-global share permissions is set.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "projectRole")]
    ProjectRole,
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "authenticated")]
    Authenticated,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::User
    }
}

