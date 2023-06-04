/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RemoteIssueLinkObject : Details of the item linked to.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteIssueLinkObject {
    #[serde(rename = "icon", skip_serializing_if = "Option::is_none")]
    pub icon: Option<Box<crate::models::RemoteObjectIcon>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<crate::models::RemoteObjectStatus>>,
    /// The summary details of the item.
    #[serde(rename = "summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// The title of the item.
    #[serde(rename = "title")]
    pub title: String,
    /// The URL of the item.
    #[serde(rename = "url")]
    pub url: String,
}

impl RemoteIssueLinkObject {
    /// Details of the item linked to.
    pub fn new(title: String, url: String) -> RemoteIssueLinkObject {
        RemoteIssueLinkObject {
            icon: None,
            status: None,
            summary: None,
            title,
            url,
        }
    }
}


