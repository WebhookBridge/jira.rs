/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// WorkflowRulesSearch : Details of the workflow and its transition rules.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkflowRulesSearch {
    /// Use expand to include additional information in the response. This parameter accepts `transition` which, for each rule, returns information about the transition the rule is assigned to.
    #[serde(rename = "expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    /// The list of workflow rule IDs.
    #[serde(rename = "ruleIds")]
    pub rule_ids: Vec<uuid::Uuid>,
    /// The workflow ID.
    #[serde(rename = "workflowEntityId")]
    pub workflow_entity_id: uuid::Uuid,
}

impl WorkflowRulesSearch {
    /// Details of the workflow and its transition rules.
    pub fn new(rule_ids: Vec<uuid::Uuid>, workflow_entity_id: uuid::Uuid) -> WorkflowRulesSearch {
        WorkflowRulesSearch {
            expand: None,
            rule_ids,
            workflow_entity_id,
        }
    }
}
