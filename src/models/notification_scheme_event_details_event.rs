/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// NotificationSchemeEventDetailsEvent : The ID of the event.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotificationSchemeEventDetailsEvent {
    /// The ID of the notification scheme event.
    #[serde(rename = "id")]
    pub id: String,
}

impl NotificationSchemeEventDetailsEvent {
    /// The ID of the event.
    pub fn new(id: String) -> NotificationSchemeEventDetailsEvent {
        NotificationSchemeEventDetailsEvent { id }
    }
}
