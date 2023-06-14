/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// LicenseMetric : A license metric

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LicenseMetric {
    /// The key of the license metric.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The value for the license metric.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl LicenseMetric {
    /// A license metric
    pub fn new() -> LicenseMetric {
        LicenseMetric {
            key: None,
            value: None,
        }
    }
}
