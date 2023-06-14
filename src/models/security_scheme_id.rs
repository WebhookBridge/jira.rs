/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SecuritySchemeId : The ID of the issue security scheme.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecuritySchemeId {
    /// The ID of the issue security scheme.
    #[serde(rename = "id")]
    pub id: String,
}

impl SecuritySchemeId {
    /// The ID of the issue security scheme.
    pub fn new(id: String) -> SecuritySchemeId {
        SecuritySchemeId { id }
    }
}
