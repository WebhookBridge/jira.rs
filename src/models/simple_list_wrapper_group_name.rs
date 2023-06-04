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
pub struct SimpleListWrapperGroupName {
    #[serde(rename = "callback", skip_serializing_if = "Option::is_none")]
    pub callback: Option<serde_json::Value>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<crate::models::GroupName>>,
    #[serde(rename = "max-results", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<serde_json::Value>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl SimpleListWrapperGroupName {
    pub fn new() -> SimpleListWrapperGroupName {
        SimpleListWrapperGroupName {
            callback: None,
            items: None,
            max_results: None,
            paging_callback: None,
            size: None,
        }
    }
}


