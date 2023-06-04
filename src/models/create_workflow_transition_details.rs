/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// CreateWorkflowTransitionDetails : The details of a workflow transition.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateWorkflowTransitionDetails {
    /// The description of the transition. The maximum length is 1000 characters.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The statuses the transition can start from.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<String>>,
    /// The name of the transition. The maximum length is 60 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The properties of the transition.
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Box<crate::models::CreateWorkflowTransitionDetailsRules>>,
    #[serde(rename = "screen", skip_serializing_if = "Option::is_none")]
    pub screen: Option<Box<crate::models::CreateWorkflowTransitionDetailsScreen>>,
    /// The status the transition goes to.
    #[serde(rename = "to")]
    pub to: String,
    /// The type of the transition.
    #[serde(rename = "type")]
    pub r#type: RHashType,
}

impl CreateWorkflowTransitionDetails {
    /// The details of a workflow transition.
    pub fn new(name: String, to: String, r#type: RHashType) -> CreateWorkflowTransitionDetails {
        CreateWorkflowTransitionDetails {
            description: None,
            from: None,
            name,
            properties: None,
            rules: None,
            screen: None,
            to,
            r#type,
        }
    }
}

/// The type of the transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RHashType {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "initial")]
    Initial,
    #[serde(rename = "directed")]
    Directed,
}

impl Default for RHashType {
    fn default() -> RHashType {
        Self::Global
    }
}
