/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ProjectFeatureState : Details of the feature state.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProjectFeatureState {
    /// The feature state.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
}

impl ProjectFeatureState {
    /// Details of the feature state.
    pub fn new() -> ProjectFeatureState {
        ProjectFeatureState {
            state: None,
        }
    }
}

/// The feature state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "COMING_SOON")]
    ComingSoon,
}

impl Default for State {
    fn default() -> State {
        Self::Enabled
    }
}

