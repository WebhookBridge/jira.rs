/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// FoundUsers : The list of users found in a search, including header text (Showing X of Y matching users) and total of matched users.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FoundUsers {
    /// Header text indicating the number of users in the response and the total number of users found in the search.
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    /// The total number of users found in the search.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<crate::models::UserPickerUser>>,
}

impl FoundUsers {
    /// The list of users found in a search, including header text (Showing X of Y matching users) and total of matched users.
    pub fn new() -> FoundUsers {
        FoundUsers {
            header: None,
            total: None,
            users: None,
        }
    }
}

