use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    CancelEmailResult, Email, EmailList, EmailStats, EmailStatsResponse, ListEmailsParams, SendEmailParams,
};

/// Emails API resource
#[derive(Debug, Clone)]
pub struct Emails {
    client: HttpClient,
}

impl Emails {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Send an email
    pub async fn send(&self, params: &SendEmailParams) -> Result<Email> {
        self.client.post("/emails", params).await
    }

    /// Get an email by ID
    pub async fn get(&self, id: &str) -> Result<Email> {
        self.client.get(&format!("/emails/{}", id)).await
    }

    /// List emails with optional filters
    pub async fn list(&self, params: &ListEmailsParams) -> Result<EmailList> {
        self.client.get_with_params("/emails", params).await
    }

    /// Get email statistics
    pub async fn stats(&self) -> Result<EmailStats> {
        let response: EmailStatsResponse = self.client.get("/emails/stats").await?;
        Ok(response.stats)
    }

    /// Cancel a pending email
    pub async fn cancel(&self, id: &str) -> Result<CancelEmailResult> {
        self.client
            .post_empty(&format!("/emails/{}/cancel", id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, Emails) {
        let mock_server = MockServer::start().await;
        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();
        let emails = Emails::new(client);
        (mock_server, emails)
    }

    #[tokio::test]
    async fn test_send_email() {
        let (mock_server, emails) = setup().await;

        Mock::given(method("POST"))
            .and(path("/emails"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "email_123",
                "from": "sender@example.com",
                "to": ["recipient@example.com"],
                "subject": "Hello",
                "status": "queued",
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = SendEmailParams {
            from: "sender@example.com".to_string(),
            to: vec!["recipient@example.com".to_string()],
            subject: Some("Hello".to_string()),
            html: Some("<p>Hello!</p>".to_string()),
            ..Default::default()
        };

        let email = emails.send(&params).await.unwrap();
        assert_eq!(email.id, "email_123");
        assert_eq!(email.status, crate::types::EmailStatus::Queued);
    }

    #[tokio::test]
    async fn test_get_email() {
        let (mock_server, emails) = setup().await;

        Mock::given(method("GET"))
            .and(path("/emails/email_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "email_123",
                "from": "sender@example.com",
                "to": ["recipient@example.com"],
                "status": "delivered",
                "created_at": "2024-01-01T00:00:00Z",
                "delivered_at": "2024-01-01T00:01:00Z"
            })))
            .mount(&mock_server)
            .await;

        let email = emails.get("email_123").await.unwrap();
        assert_eq!(email.id, "email_123");
        assert_eq!(email.status, crate::types::EmailStatus::Delivered);
    }

    #[tokio::test]
    async fn test_list_emails() {
        let (mock_server, emails) = setup().await;

        Mock::given(method("GET"))
            .and(path("/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {"id": "email_1", "from": "a@example.com", "to": ["b@example.com"], "status": "sent", "created_at": "2024-01-01T00:00:00Z"},
                    {"id": "email_2", "from": "a@example.com", "to": ["c@example.com"], "status": "delivered", "created_at": "2024-01-01T00:00:00Z"}
                ],
                "meta": {"page": 1, "limit": 10, "total": 2, "total_pages": 1}
            })))
            .mount(&mock_server)
            .await;

        let result = emails.list(&ListEmailsParams::default()).await.unwrap();
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.meta.total, 2);
    }

    #[tokio::test]
    async fn test_get_stats() {
        let (mock_server, emails) = setup().await;

        Mock::given(method("GET"))
            .and(path("/emails/stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "stats": {
                    "total": 1000,
                    "sent": 950,
                    "failed": 50,
                    "transactional": 600,
                    "marketing": 400,
                    "successRate": 95.0
                }
            })))
            .mount(&mock_server)
            .await;

        let stats = emails.stats().await.unwrap();
        assert_eq!(stats.total, 1000);
        assert_eq!(stats.sent, 950);
    }

    #[tokio::test]
    async fn test_cancel_email() {
        let (mock_server, emails) = setup().await;

        Mock::given(method("POST"))
            .and(path("/emails/email_123/cancel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "email_123",
                "cancelled": true
            })))
            .mount(&mock_server)
            .await;

        let result = emails.cancel("email_123").await.unwrap();
        assert!(result.cancelled);
    }
}
