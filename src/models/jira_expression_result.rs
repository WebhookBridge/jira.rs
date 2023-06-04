/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionResult : The result of evaluating a Jira expression.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionResult {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::JiraExpressionResultMeta>>,
    /// The value of the evaluated expression. It may be a primitive JSON value or a Jira REST API object. (Some expressions do not produce any meaningful results—for example, an expression that returns a lambda function—if that's the case a simple string representation is returned. These string representations should not be relied upon and may change without notice.)
    #[serde(rename = "value", deserialize_with = "Option::deserialize")]
    pub value: Option<serde_json::Value>,
}

impl JiraExpressionResult {
    /// The result of evaluating a Jira expression.
    pub fn new(value: Option<serde_json::Value>) -> JiraExpressionResult {
        JiraExpressionResult {
            meta: None,
            value,
        }
    }
}


