/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SetDefaultResolutionRequest : The new default issue resolution.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDefaultResolutionRequest {
    /// The ID of the new default issue resolution. Must be an existing ID or null. Setting this to null erases the default resolution setting.
    #[serde(rename = "id")]
    pub id: String,
}

impl SetDefaultResolutionRequest {
    /// The new default issue resolution.
    pub fn new(id: String) -> SetDefaultResolutionRequest {
        SetDefaultResolutionRequest {
            id,
        }
    }
}


