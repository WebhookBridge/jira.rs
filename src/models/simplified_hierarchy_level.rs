/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SimplifiedHierarchyLevel {
    /// The ID of the level above this one in the hierarchy. This property is deprecated, see [Change notice: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/).
    #[serde(rename = "aboveLevelId", skip_serializing_if = "Option::is_none")]
    pub above_level_id: Option<i64>,
    /// The ID of the level below this one in the hierarchy. This property is deprecated, see [Change notice: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/).
    #[serde(rename = "belowLevelId", skip_serializing_if = "Option::is_none")]
    pub below_level_id: Option<i64>,
    /// The external UUID of the hierarchy level. This property is deprecated, see [Change notice: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/).
    #[serde(rename = "externalUuid", skip_serializing_if = "Option::is_none")]
    pub external_uuid: Option<uuid::Uuid>,
    #[serde(
        rename = "hierarchyLevelNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub hierarchy_level_number: Option<i32>,
    /// The ID of the hierarchy level. This property is deprecated, see [Change notice: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/).
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The issue types available in this hierarchy level.
    #[serde(rename = "issueTypeIds", skip_serializing_if = "Option::is_none")]
    pub issue_type_ids: Option<Vec<i64>>,
    /// The level of this item in the hierarchy.
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// The name of this hierarchy level.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the project configuration. This property is deprecated, see [Change oticen: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/).
    #[serde(
        rename = "projectConfigurationId",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_configuration_id: Option<i64>,
}

impl SimplifiedHierarchyLevel {
    pub fn new() -> SimplifiedHierarchyLevel {
        SimplifiedHierarchyLevel {
            above_level_id: None,
            below_level_id: None,
            external_uuid: None,
            hierarchy_level_number: None,
            id: None,
            issue_type_ids: None,
            level: None,
            name: None,
            project_configuration_id: None,
        }
    }
}
