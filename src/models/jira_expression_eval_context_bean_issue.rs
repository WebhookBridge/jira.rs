/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionEvalContextBeanIssue : The issue that is available under the `issue` variable when evaluating the expression.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionEvalContextBeanIssue {
    /// The ID of the referenced item.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The key of the referenced item.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl JiraExpressionEvalContextBeanIssue {
    /// The issue that is available under the `issue` variable when evaluating the expression.
    pub fn new() -> JiraExpressionEvalContextBeanIssue {
        JiraExpressionEvalContextBeanIssue {
            id: None,
            key: None,
        }
    }
}

