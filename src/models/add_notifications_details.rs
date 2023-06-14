/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AddNotificationsDetails : Details of notifications which should be added to the notification scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddNotificationsDetails {
    /// The list of notifications which should be added to the notification scheme.
    #[serde(rename = "notificationSchemeEvents")]
    pub notification_scheme_events: Vec<crate::models::NotificationSchemeEventDetails>,
}

impl AddNotificationsDetails {
    /// Details of notifications which should be added to the notification scheme.
    pub fn new(
        notification_scheme_events: Vec<crate::models::NotificationSchemeEventDetails>,
    ) -> AddNotificationsDetails {
        AddNotificationsDetails {
            notification_scheme_events,
        }
    }
}
