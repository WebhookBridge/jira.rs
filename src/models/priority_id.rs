/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PriorityId : The ID of an issue priority.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PriorityId {
    /// The ID of the issue priority.
    #[serde(rename = "id")]
    pub id: String,
}

impl PriorityId {
    /// The ID of an issue priority.
    pub fn new(id: String) -> PriorityId {
        PriorityId { id }
    }
}
