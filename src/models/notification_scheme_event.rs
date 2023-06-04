/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// NotificationSchemeEvent : Details about a notification scheme event.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEvent {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub event: Option<Box<crate::models::NotificationEvent>>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<crate::models::EventNotification>>,
}

impl NotificationSchemeEvent {
    /// Details about a notification scheme event.
    pub fn new() -> NotificationSchemeEvent {
        NotificationSchemeEvent {
            event: None,
            notifications: None,
        }
    }
}


