/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AssociatedItemBean : Details of an item associated with the changed record.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AssociatedItemBean {
    /// The ID of the associated record.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the associated record.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the associated parent record.
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The name of the associated parent record.
    #[serde(rename = "parentName", skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    /// The type of the associated record.
    #[serde(rename = "typeName", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

impl AssociatedItemBean {
    /// Details of an item associated with the changed record.
    pub fn new() -> AssociatedItemBean {
        AssociatedItemBean {
            id: None,
            name: None,
            parent_id: None,
            parent_name: None,
            type_name: None,
        }
    }
}


