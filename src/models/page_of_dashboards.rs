/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PageOfDashboards : A page containing dashboard details.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PageOfDashboards {
    /// List of dashboards.
    #[serde(rename = "dashboards", skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<crate::models::Dashboard>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The URL of the next page of results, if any.
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// The URL of the previous page of results, if any.
    #[serde(rename = "prev", skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The number of results on the page.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl PageOfDashboards {
    /// A page containing dashboard details.
    pub fn new() -> PageOfDashboards {
        PageOfDashboards {
            dashboards: None,
            max_results: None,
            next: None,
            prev: None,
            start_at: None,
            total: None,
        }
    }
}
