/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueReadOnly : The default text for a read only custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueReadOnly {
    /// The default text. The maximum length is 255 characters.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
}

impl CustomFieldContextDefaultValueReadOnly {
    /// The default text for a read only custom field.
    pub fn new(r#type: String) -> CustomFieldContextDefaultValueReadOnly {
        CustomFieldContextDefaultValueReadOnly {
            text: None,
            r#type,
        }
    }
}


