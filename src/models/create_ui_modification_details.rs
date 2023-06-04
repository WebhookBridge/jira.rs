/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateUiModificationDetails : The details of a UI modification.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUiModificationDetails {
    /// List of contexts of the UI modification. The maximum number of contexts is 1000.
    #[serde(rename = "contexts", skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<crate::models::UiModificationContextDetails>>,
    /// The data of the UI modification. The maximum size of the data is 50000 characters.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The description of the UI modification. The maximum length is 255 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the UI modification. The maximum length is 255 characters.
    #[serde(rename = "name")]
    pub name: String,
}

impl CreateUiModificationDetails {
    /// The details of a UI modification.
    pub fn new(name: String) -> CreateUiModificationDetails {
        CreateUiModificationDetails {
            contexts: None,
            data: None,
            description: None,
            name,
        }
    }
}

