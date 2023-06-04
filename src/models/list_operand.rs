/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// ListOperand : An operand that is a list of values.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListOperand {
    /// Encoded operand, which can be used directly in a JQL query.
    #[serde(rename = "encodedOperand", skip_serializing_if = "Option::is_none")]
    pub encoded_operand: Option<String>,
    /// The list of operand values.
    #[serde(rename = "values")]
    pub values: Vec<crate::models::JqlQueryUnitaryOperand>,
}

impl ListOperand {
    /// An operand that is a list of values.
    pub fn new(values: Vec<crate::models::JqlQueryUnitaryOperand>) -> ListOperand {
        ListOperand {
            encoded_operand: None,
            values,
        }
    }
}


