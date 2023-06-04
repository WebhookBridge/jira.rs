/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreatedIssue : Details about a created issue or subtask.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatedIssue {
    /// The ID of the created issue or subtask.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The key of the created issue or subtask.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The URL of the created issue or subtask.
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub param_self: Option<String>,
    #[serde(rename = "transition", skip_serializing_if = "Option::is_none")]
    pub transition: Option<Box<crate::models::CreatedIssueTransition>>,
    #[serde(rename = "watchers", skip_serializing_if = "Option::is_none")]
    pub watchers: Option<Box<crate::models::CreatedIssueWatchers>>,
}

impl CreatedIssue {
    /// Details about a created issue or subtask.
    pub fn new() -> CreatedIssue {
        CreatedIssue {
            id: None,
            key: None,
            param_self: None,
            transition: None,
            watchers: None,
        }
    }
}


