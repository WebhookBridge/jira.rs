/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// DashboardGadgetSettings : Details of the settings for a dashboard gadget.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardGadgetSettings {
    /// The color of the gadget. Should be one of `blue`, `red`, `yellow`, `green`, `cyan`, `purple`, `gray`, or `white`.
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Whether to ignore the validation of module key and URI. For example, when a gadget is created that is a part of an application that isn't installed.
    #[serde(rename = "ignoreUriAndModuleKeyValidation", skip_serializing_if = "Option::is_none")]
    pub ignore_uri_and_module_key_validation: Option<bool>,
    /// The module key of the gadget type. Can't be provided with `uri`.
    #[serde(rename = "moduleKey", skip_serializing_if = "Option::is_none")]
    pub module_key: Option<String>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<crate::models::DashboardGadgetSettingsPosition>>,
    /// The title of the gadget.
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URI of the gadget type. Can't be provided with `moduleKey`.
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl DashboardGadgetSettings {
    /// Details of the settings for a dashboard gadget.
    pub fn new() -> DashboardGadgetSettings {
        DashboardGadgetSettings {
            color: None,
            ignore_uri_and_module_key_validation: None,
            module_key: None,
            position: None,
            title: None,
            uri: None,
        }
    }
}


