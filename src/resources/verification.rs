use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{BatchVerificationResult, VerificationResult, VerificationStats};
use serde::Serialize;

/// Verification API resource
#[derive(Debug, Clone)]
pub struct Verification {
    client: HttpClient,
}

#[derive(Serialize)]
struct VerifyRequest {
    email: String,
}

#[derive(Serialize)]
struct BatchVerifyRequest {
    emails: Vec<String>,
}

impl Verification {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Verify a single email address
    pub async fn verify(&self, email: &str) -> Result<VerificationResult> {
        self.client
            .post(
                "/verification/verify",
                &VerifyRequest {
                    email: email.to_string(),
                },
            )
            .await
    }

    /// Verify multiple email addresses in batch
    pub async fn verify_batch(&self, emails: Vec<String>) -> Result<BatchVerificationResult> {
        self.client
            .post("/verification/batch", &BatchVerifyRequest { emails })
            .await
    }

    /// Get batch verification status
    pub async fn get_batch_status(&self, verification_id: &str) -> Result<BatchVerificationResult> {
        self.client
            .get(&format!("/verification/batch/{}", verification_id))
            .await
    }

    /// Get verification statistics
    pub async fn stats(&self) -> Result<VerificationStats> {
        self.client.get("/verification/stats").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use crate::types::VerificationStatus;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, Verification) {
        let mock_server = MockServer::start().await;
        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();
        let verification = Verification::new(client);
        (mock_server, verification)
    }

    #[tokio::test]
    async fn test_verify_valid_email() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("POST"))
            .and(path("/verification/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "email": "valid@example.com",
                "status": "valid",
                "is_valid": true,
                "is_disposable": false,
                "is_role_based": false,
                "is_free_provider": false,
                "mx_found": true,
                "smtp_check": true
            })))
            .mount(&mock_server)
            .await;

        let result = verification.verify("valid@example.com").await.unwrap();
        assert_eq!(result.email, "valid@example.com");
        assert_eq!(result.status, VerificationStatus::Valid);
        assert!(result.is_valid);
    }

    #[tokio::test]
    async fn test_verify_invalid_email() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("POST"))
            .and(path("/verification/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "email": "invalid@nonexistent.domain",
                "status": "invalid",
                "is_valid": false,
                "is_disposable": false,
                "is_role_based": false,
                "is_free_provider": false,
                "mx_found": false
            })))
            .mount(&mock_server)
            .await;

        let result = verification
            .verify("invalid@nonexistent.domain")
            .await
            .unwrap();
        assert!(!result.is_valid);
        assert!(!result.mx_found);
    }

    #[tokio::test]
    async fn test_verify_with_suggestion() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("POST"))
            .and(path("/verification/verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "email": "user@gmial.com",
                "status": "invalid",
                "is_valid": false,
                "is_disposable": false,
                "is_role_based": false,
                "is_free_provider": false,
                "mx_found": false,
                "suggestion": "user@gmail.com"
            })))
            .mount(&mock_server)
            .await;

        let result = verification.verify("user@gmial.com").await.unwrap();
        assert_eq!(result.suggestion, Some("user@gmail.com".to_string()));
    }

    #[tokio::test]
    async fn test_verify_batch() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("POST"))
            .and(path("/verification/batch"))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "verification_id": "batch_123",
                "status": "processing",
                "total": 3,
                "processed": 0,
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let emails = vec![
            "email1@example.com".to_string(),
            "email2@example.com".to_string(),
            "email3@example.com".to_string(),
        ];

        let result = verification.verify_batch(emails).await.unwrap();
        assert_eq!(result.verification_id, "batch_123");
        assert_eq!(result.status, "processing");
        assert_eq!(result.total, 3);
    }

    #[tokio::test]
    async fn test_get_batch_status() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("GET"))
            .and(path("/verification/batch/batch_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "verification_id": "batch_123",
                "status": "completed",
                "total": 3,
                "processed": 3,
                "results": [
                    {"email": "email1@example.com", "status": "valid", "is_valid": true, "is_disposable": false, "is_role_based": false, "is_free_provider": false, "mx_found": true},
                    {"email": "email2@example.com", "status": "invalid", "is_valid": false, "is_disposable": false, "is_role_based": false, "is_free_provider": false, "mx_found": false},
                    {"email": "email3@example.com", "status": "risky", "is_valid": true, "is_disposable": true, "is_role_based": false, "is_free_provider": false, "mx_found": true}
                ],
                "created_at": "2024-01-01T00:00:00Z",
                "completed_at": "2024-01-01T00:01:00Z"
            })))
            .mount(&mock_server)
            .await;

        let result = verification.get_batch_status("batch_123").await.unwrap();
        assert_eq!(result.status, "completed");
        assert_eq!(result.processed, 3);
        assert!(result.results.is_some());
        assert_eq!(result.results.unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_verification_stats() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("GET"))
            .and(path("/verification/stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total_verified": 10000,
                "valid_count": 8500,
                "invalid_count": 1000,
                "risky_count": 400,
                "unknown_count": 100
            })))
            .mount(&mock_server)
            .await;

        let stats = verification.stats().await.unwrap();
        assert_eq!(stats.total_verified, 10000);
        assert_eq!(stats.valid_count, 8500);
    }
}
