/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// PageOfChangelogs : A page of changelogs.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PageOfChangelogs {
    /// The list of changelogs.
    #[serde(rename = "histories", skip_serializing_if = "Option::is_none")]
    pub histories: Option<Vec<crate::models::Changelog>>,
    /// The maximum number of results that could be on the page.
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// The index of the first item returned on the page.
    #[serde(rename = "startAt", skip_serializing_if = "Option::is_none")]
    pub start_at: Option<i32>,
    /// The number of results on the page.
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

impl PageOfChangelogs {
    /// A page of changelogs.
    pub fn new() -> PageOfChangelogs {
        PageOfChangelogs {
            histories: None,
            max_results: None,
            start_at: None,
            total: None,
        }
    }
}

