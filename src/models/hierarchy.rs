/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// Hierarchy : The project issue type hierarchy.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Hierarchy {
    /// The ID of the base level. This property is deprecated, see [Change notice: Removing hierarchy level IDs from next-gen APIs](https://developer.atlassian.com/cloud/jira/platform/change-notice-removing-hierarchy-level-ids-from-next-gen-apis/).
    #[serde(rename = "baseLevelId", skip_serializing_if = "Option::is_none")]
    pub base_level_id: Option<i64>,
    /// Details about the hierarchy level.
    #[serde(rename = "levels", skip_serializing_if = "Option::is_none")]
    pub levels: Option<Vec<crate::models::SimplifiedHierarchyLevel>>,
}

impl Hierarchy {
    /// The project issue type hierarchy.
    pub fn new() -> Hierarchy {
        Hierarchy {
            base_level_id: None,
            levels: None,
        }
    }
}

