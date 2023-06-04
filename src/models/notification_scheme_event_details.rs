/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// NotificationSchemeEventDetails : Details of a notification scheme event.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEventDetails {
    #[serde(rename = "event")]
    pub event: Box<crate::models::NotificationSchemeEventDetailsEvent>,
    /// The list of notifications mapped to a specified event.
    #[serde(rename = "notifications")]
    pub notifications: Vec<crate::models::NotificationSchemeNotificationDetails>,
}

impl NotificationSchemeEventDetails {
    /// Details of a notification scheme event.
    pub fn new(event: crate::models::NotificationSchemeEventDetailsEvent, notifications: Vec<crate::models::NotificationSchemeNotificationDetails>) -> NotificationSchemeEventDetails {
        NotificationSchemeEventDetails {
            event: Box::new(event),
            notifications,
        }
    }
}

