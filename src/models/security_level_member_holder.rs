/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityLevelMemberHolder : The user or group being granted the permission. It consists of a `type` and a type-dependent `parameter`. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityLevelMemberHolder {
    /// Expand options that include additional permission holder details in the response.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// As a group's name can change, use of `value` is recommended. The identifier associated withthe `type` value that defines the holder of the permission.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// The type of permission holder.
    #[serde(rename = "type")]
    pub r#type: String,
    /// The identifier associated with the `type` value that defines the holder of the permission.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SecurityLevelMemberHolder {
    /// The user or group being granted the permission. It consists of a `type` and a type-dependent `parameter`. See [Holder object](../api-group-permission-schemes/#holder-object) in *Get all permission schemes* for more information.
    pub fn new(r#type: String) -> SecurityLevelMemberHolder {
        SecurityLevelMemberHolder {
            expand: None,
            parameter: None,
            r#type,
            value: None,
        }
    }
}


