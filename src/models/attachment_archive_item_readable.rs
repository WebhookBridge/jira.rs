/*
 * The Jira Cloud platform REST API
 *
 * Jira Cloud platform REST API documentation
 *
 * The version of the OpenAPI document: 1001.0.0-SNAPSHOT
 * Contact: ecosystem@atlassian.com
 * Generated by: https://openapi-generator.tech
 */

/// AttachmentArchiveItemReadable : Metadata for an item in an attachment archive.

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AttachmentArchiveItemReadable {
    /// The position of the item within the archive.
    #[serde(rename = "index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    /// The label for the archive item.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The MIME type of the archive item.
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The path of the archive item.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The size of the archive item.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
}

impl AttachmentArchiveItemReadable {
    /// Metadata for an item in an attachment archive.
    pub fn new() -> AttachmentArchiveItemReadable {
        AttachmentArchiveItemReadable {
            index: None,
            label: None,
            media_type: None,
            path: None,
            size: None,
        }
    }
}
