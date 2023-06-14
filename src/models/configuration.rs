/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Configuration : Details about the configuration of Jira.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Configuration {
    /// Whether the ability to add attachments to issues is enabled.
    #[serde(rename = "attachmentsEnabled", skip_serializing_if = "Option::is_none")]
    pub attachments_enabled: Option<bool>,
    /// Whether the ability to link issues is enabled.
    #[serde(
        rename = "issueLinkingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub issue_linking_enabled: Option<bool>,
    /// Whether the ability to create subtasks for issues is enabled.
    #[serde(rename = "subTasksEnabled", skip_serializing_if = "Option::is_none")]
    pub sub_tasks_enabled: Option<bool>,
    #[serde(
        rename = "timeTrackingConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_tracking_configuration:
        Option<Box<crate::models::ConfigurationTimeTrackingConfiguration>>,
    /// Whether the ability to track time is enabled. This property is deprecated.
    #[serde(
        rename = "timeTrackingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub time_tracking_enabled: Option<bool>,
    /// Whether the ability to create unassigned issues is enabled. See [Configuring Jira application options](https://confluence.atlassian.com/x/uYXKM) for details.
    #[serde(
        rename = "unassignedIssuesAllowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub unassigned_issues_allowed: Option<bool>,
    /// Whether the ability for users to vote on issues is enabled. See [Configuring Jira application options](https://confluence.atlassian.com/x/uYXKM) for details.
    #[serde(rename = "votingEnabled", skip_serializing_if = "Option::is_none")]
    pub voting_enabled: Option<bool>,
    /// Whether the ability for users to watch issues is enabled. See [Configuring Jira application options](https://confluence.atlassian.com/x/uYXKM) for details.
    #[serde(rename = "watchingEnabled", skip_serializing_if = "Option::is_none")]
    pub watching_enabled: Option<bool>,
}

impl Configuration {
    /// Details about the configuration of Jira.
    pub fn new() -> Configuration {
        Configuration {
            attachments_enabled: None,
            issue_linking_enabled: None,
            sub_tasks_enabled: None,
            time_tracking_configuration: None,
            time_tracking_enabled: None,
            unassigned_issues_allowed: None,
            voting_enabled: None,
            watching_enabled: None,
        }
    }
}
