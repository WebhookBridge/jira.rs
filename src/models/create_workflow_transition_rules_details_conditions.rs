/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateWorkflowTransitionRulesDetailsConditions : The workflow conditions.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWorkflowTransitionRulesDetailsConditions {
    /// The list of workflow conditions.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<crate::models::CreateWorkflowCondition>>,
    /// EXPERIMENTAL. The configuration of the transition rule.
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<::std::collections::HashMap<String, serde_json::Value>>,
    /// The compound condition operator.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<Operator>,
    /// The type of the transition rule.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl CreateWorkflowTransitionRulesDetailsConditions {
    /// The workflow conditions.
    pub fn new() -> CreateWorkflowTransitionRulesDetailsConditions {
        CreateWorkflowTransitionRulesDetailsConditions {
            conditions: None,
            configuration: None,
            operator: None,
            r#type: None,
        }
    }
}

/// The compound condition operator.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Operator {
    #[serde(rename = "AND")]
    And,
    #[serde(rename = "OR")]
    Or,
}

impl Default for Operator {
    fn default() -> Operator {
        Self::And
    }
}

