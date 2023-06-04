/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ParsedJqlQueryStructure : The syntax tree of the query. Empty if the query was invalid.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ParsedJqlQueryStructure {
    #[serde(rename = "orderBy", skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Box<crate::models::JqlQueryOrderByClause>>,
    #[serde(rename = "where", skip_serializing_if = "Option::is_none")]
    pub r#where: Option<Box<crate::models::JqlQueryClause>>,
}

impl ParsedJqlQueryStructure {
    /// The syntax tree of the query. Empty if the query was invalid.
    pub fn new() -> ParsedJqlQueryStructure {
        ParsedJqlQueryStructure {
            order_by: None,
            r#where: None,
        }
    }
}

