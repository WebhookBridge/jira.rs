/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// RuleConfiguration : A rule configuration.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RuleConfiguration {
    /// EXPERIMENTAL: Whether the rule is disabled.
    #[serde(rename = "disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    /// EXPERIMENTAL: A tag used to filter rules in [Get workflow transition rule configurations](https://developer.atlassian.com/cloud/jira/platform/rest/v3/api-group-workflow-transition-rules/#api-rest-api-3-workflow-rule-config-get).
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Configuration of the rule, as it is stored by the Connect or the Forge app on the rule configuration page.
    #[serde(rename = "value")]
    pub value: String,
}

impl RuleConfiguration {
    /// A rule configuration.
    pub fn new(value: String) -> RuleConfiguration {
        RuleConfiguration {
            disabled: None,
            tag: None,
            value,
        }
    }
}


