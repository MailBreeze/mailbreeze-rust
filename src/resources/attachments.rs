use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{Attachment, CreateUploadParams, UploadUrl};

/// Attachments API resource
#[derive(Debug, Clone)]
pub struct Attachments {
    client: HttpClient,
}

impl Attachments {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Create a pre-signed upload URL
    pub async fn create_upload_url(&self, params: &CreateUploadParams) -> Result<UploadUrl> {
        self.client.post("/attachments/upload", params).await
    }

    /// Get an attachment by ID
    pub async fn get(&self, id: &str) -> Result<Attachment> {
        self.client.get(&format!("/attachments/{}", id)).await
    }

    /// Delete an attachment
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client.delete(&format!("/attachments/{}", id)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, Attachments) {
        let mock_server = MockServer::start().await;
        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();
        let attachments = Attachments::new(client);
        (mock_server, attachments)
    }

    #[tokio::test]
    async fn test_create_upload_url() {
        let (mock_server, attachments) = setup().await;

        Mock::given(method("POST"))
            .and(path("/attachments/upload"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "attachment_id": "attach_123",
                "upload_url": "https://storage.example.com/upload/abc123",
                "expires_at": "2024-01-01T01:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = CreateUploadParams {
            filename: "document.pdf".to_string(),
            content_type: "application/pdf".to_string(),
            size: 1024000,
        };

        let result = attachments.create_upload_url(&params).await.unwrap();
        assert_eq!(result.attachment_id, "attach_123");
        assert!(result.upload_url.starts_with("https://"));
    }

    #[tokio::test]
    async fn test_get_attachment() {
        let (mock_server, attachments) = setup().await;

        Mock::given(method("GET"))
            .and(path("/attachments/attach_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "attach_123",
                "filename": "document.pdf",
                "content_type": "application/pdf",
                "size": 1024000,
                "status": "ready",
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let attachment = attachments.get("attach_123").await.unwrap();
        assert_eq!(attachment.id, "attach_123");
        assert_eq!(attachment.filename, "document.pdf");
        assert_eq!(attachment.status, "ready");
    }

    #[tokio::test]
    async fn test_delete_attachment() {
        let (mock_server, attachments) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/attachments/attach_123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        attachments.delete("attach_123").await.unwrap();
    }

    #[tokio::test]
    async fn test_create_image_upload_url() {
        let (mock_server, attachments) = setup().await;

        Mock::given(method("POST"))
            .and(path("/attachments/upload"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "attachment_id": "attach_456",
                "upload_url": "https://storage.example.com/upload/xyz789",
                "expires_at": "2024-01-01T01:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = CreateUploadParams {
            filename: "logo.png".to_string(),
            content_type: "image/png".to_string(),
            size: 50000,
        };

        let result = attachments.create_upload_url(&params).await.unwrap();
        assert_eq!(result.attachment_id, "attach_456");
    }

    #[tokio::test]
    async fn test_get_pending_attachment() {
        let (mock_server, attachments) = setup().await;

        Mock::given(method("GET"))
            .and(path("/attachments/attach_789"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "attach_789",
                "filename": "video.mp4",
                "content_type": "video/mp4",
                "size": 10000000,
                "status": "pending",
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let attachment = attachments.get("attach_789").await.unwrap();
        assert_eq!(attachment.status, "pending");
    }
}
