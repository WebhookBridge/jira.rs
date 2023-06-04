/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// UiModificationIdentifiers : Identifiers for a UI modification.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UiModificationIdentifiers {
    /// The ID of the UI modification.
    #[serde(rename = "id")]
    pub id: String,
    /// The URL of the UI modification.
    #[serde(rename = "self")]
    pub param_self: String,
}

impl UiModificationIdentifiers {
    /// Identifiers for a UI modification.
    pub fn new(id: String, param_self: String) -> UiModificationIdentifiers {
        UiModificationIdentifiers {
            id,
            param_self,
        }
    }
}

