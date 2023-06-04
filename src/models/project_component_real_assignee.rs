/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectComponentRealAssignee : The user assigned to issues created with this component, when `assigneeType` does not identify a valid assignee.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectComponentRealAssignee {
    /// The account ID of the user, which uniquely identifies the user across all Atlassian products. For example, *5b10ac8d82e05b22cc7d4ef5*. Required in requests.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The user account type. Can take the following values:   *  `atlassian` regular Atlassian user account  *  `app` system account used for Connect applications and OAuth to represent external systems  *  `customer` Jira Service Desk account representing an external service desk
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// Whether the user is active.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "applicationRoles", skip_serializing_if = "Option::is_none")]
    pub application_roles: Option<Box<crate::models::UserApplicationRoles>>,
    #[serde(rename = "avatarUrls", skip_serializing_if = "Option::is_none")]
    pub avatar_urls: Option<Box<crate::models::UserAvatarUrls>>,
    /// The display name of the user. Depending on the user’s privacy setting, this may return an alternative value.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// The email address of the user. Depending on the user’s privacy setting, this may be returned as null.
    #[serde(rename = "emailAddress", skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Expand options that include additional user details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Box<crate::models::UserGroups>>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The locale of the user. Depending on the user’s privacy setting, this may be returned as null.
    #[serde(rename = "locale", skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// This property is no longer available and will be removed from the documentation soon. See the [deprecation notice](https://developer.atlassian.com/cloud/jira/platform/deprecation-notice-user-privacy-api-migration-guide/) for details.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL of the user.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    /// The time zone specified in the user's profile. Depending on the user’s privacy setting, this may be returned as null.
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

impl ProjectComponentRealAssignee {
    /// The user assigned to issues created with this component, when `assigneeType` does not identify a valid assignee.
    pub fn new() -> ProjectComponentRealAssignee {
        ProjectComponentRealAssignee {
            account_id: None,
            account_type: None,
            active: None,
            application_roles: None,
            avatar_urls: None,
            display_name: None,
            email_address: None,
            expand: None,
            groups: None,
            key: None,
            locale: None,
            name: None,
            param_self: None,
            time_zone: None,
        }
    }
}

/// The user account type. Can take the following values:   *  `atlassian` regular Atlassian user account  *  `app` system account used for Connect applications and OAuth to represent external systems  *  `customer` Jira Service Desk account representing an external service desk
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "atlassian")]
    Atlassian,
    #[serde(rename = "app")]
    App,
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for AccountType {
    fn default() -> AccountType {
        Self::Atlassian
    }
}

