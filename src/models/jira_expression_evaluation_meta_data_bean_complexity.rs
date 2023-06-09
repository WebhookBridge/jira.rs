/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// JiraExpressionEvaluationMetaDataBeanComplexity : Contains information about the expression complexity. For example, the number of steps it took to evaluate the expression.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JiraExpressionEvaluationMetaDataBeanComplexity {
    #[serde(rename = "beans")]
    pub beans: Box<crate::models::JiraExpressionsComplexityBeanBeans>,
    #[serde(rename = "expensiveOperations")]
    pub expensive_operations: Box<crate::models::JiraExpressionsComplexityBeanExpensiveOperations>,
    #[serde(rename = "primitiveValues")]
    pub primitive_values: Box<crate::models::JiraExpressionsComplexityBeanPrimitiveValues>,
    #[serde(rename = "steps")]
    pub steps: Box<crate::models::JiraExpressionsComplexityBeanSteps>,
}

impl JiraExpressionEvaluationMetaDataBeanComplexity {
    /// Contains information about the expression complexity. For example, the number of steps it took to evaluate the expression.
    pub fn new(
        beans: crate::models::JiraExpressionsComplexityBeanBeans,
        expensive_operations: crate::models::JiraExpressionsComplexityBeanExpensiveOperations,
        primitive_values: crate::models::JiraExpressionsComplexityBeanPrimitiveValues,
        steps: crate::models::JiraExpressionsComplexityBeanSteps,
    ) -> JiraExpressionEvaluationMetaDataBeanComplexity {
        JiraExpressionEvaluationMetaDataBeanComplexity {
            beans: Box::new(beans),
            expensive_operations: Box::new(expensive_operations),
            primitive_values: Box::new(primitive_values),
            steps: Box::new(steps),
        }
    }
}
