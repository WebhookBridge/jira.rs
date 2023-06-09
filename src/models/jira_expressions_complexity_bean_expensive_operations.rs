/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionsComplexityBeanExpensiveOperations : The number of expensive operations executed while evaluating the expression. Expensive operations are those that load additional data, such as entity properties, comments, or custom fields.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionsComplexityBeanExpensiveOperations {
    /// The maximum allowed complexity. The evaluation will fail if this value is exceeded.
    #[serde(rename = "limit")]
    pub limit: i32,
    /// The complexity value of the current expression.
    #[serde(rename = "value")]
    pub value: i32,
}

impl JiraExpressionsComplexityBeanExpensiveOperations {
    /// The number of expensive operations executed while evaluating the expression. Expensive operations are those that load additional data, such as entity properties, comments, or custom fields.
    pub fn new(limit: i32, value: i32) -> JiraExpressionsComplexityBeanExpensiveOperations {
        JiraExpressionsComplexityBeanExpensiveOperations { limit, value }
    }
}
