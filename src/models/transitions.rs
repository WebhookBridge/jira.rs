/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Transitions : List of issue transitions.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transitions {
    /// Expand options that include additional transitions details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// List of issue transitions.
    #[serde(rename = "transitions", skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<crate::models::IssueTransition>>,
}

impl Transitions {
    /// List of issue transitions.
    pub fn new() -> Transitions {
        Transitions {
            expand: None,
            transitions: None,
        }
    }
}
