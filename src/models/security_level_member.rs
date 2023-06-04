/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// SecurityLevelMember : Issue security level member.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SecurityLevelMember {
    #[serde(rename = "holder")]
    pub holder: Box<crate::models::SecurityLevelMemberHolder>,
    /// The ID of the issue security level member.
    #[serde(rename = "id")]
    pub id: String,
    /// The ID of the issue security level.
    #[serde(rename = "issueSecurityLevelId")]
    pub issue_security_level_id: String,
    /// The ID of the issue security scheme.
    #[serde(rename = "issueSecuritySchemeId")]
    pub issue_security_scheme_id: String,
}

impl SecurityLevelMember {
    /// Issue security level member.
    pub fn new(holder: crate::models::SecurityLevelMemberHolder, id: String, issue_security_level_id: String, issue_security_scheme_id: String) -> SecurityLevelMember {
        SecurityLevelMember {
            holder: Box::new(holder),
            id,
            issue_security_level_id,
            issue_security_scheme_id,
        }
    }
}


