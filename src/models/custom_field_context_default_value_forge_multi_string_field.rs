/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CustomFieldContextDefaultValueForgeMultiStringField : The default text for a Forge collection of strings custom field.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CustomFieldContextDefaultValueForgeMultiStringField {
    #[serde(rename = "type")]
    pub r#type: String,
    /// List of string values. The maximum length for a value is 254 characters.
    #[serde(rename = "values", skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl CustomFieldContextDefaultValueForgeMultiStringField {
    /// The default text for a Forge collection of strings custom field.
    pub fn new(r#type: String) -> CustomFieldContextDefaultValueForgeMultiStringField {
        CustomFieldContextDefaultValueForgeMultiStringField {
            r#type,
            values: None,
        }
    }
}


