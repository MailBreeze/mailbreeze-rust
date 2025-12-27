use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    BatchVerificationResult, VerificationListItem, VerificationListResponse, VerificationResult,
    VerificationStats,
};
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
                "/email-verification/single",
                &VerifyRequest {
                    email: email.to_string(),
                },
            )
            .await
    }

    /// Verify multiple email addresses in batch
    pub async fn batch(&self, emails: Vec<String>) -> Result<BatchVerificationResult> {
        self.client
            .post("/email-verification/batch", &BatchVerifyRequest { emails })
            .await
    }

    /// Get batch verification status
    pub async fn get(&self, verification_id: &str) -> Result<BatchVerificationResult> {
        self.client
            .get(&format!("/email-verification/{}", verification_id))
            .await
    }

    /// List verification batches
    pub async fn list(&self) -> Result<Vec<VerificationListItem>> {
        // API returns data as {items: [...]}
        let response: VerificationListResponse = self.client.get("/email-verification").await?;
        Ok(response.items)
    }

    /// Get verification statistics
    pub async fn stats(&self) -> Result<VerificationStats> {
        self.client.get("/email-verification/stats").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
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
            .and(path("/api/v1/email-verification/single"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "email": "valid@example.com",
                    "status": "valid",
                    "isValid": true,
                    "isDisposable": false,
                    "isRoleBased": false,
                    "isFreeProvider": false,
                    "mxFound": true,
                    "smtpCheck": true
                }
            })))
            .mount(&mock_server)
            .await;

        let result = verification.verify("valid@example.com").await.unwrap();
        assert_eq!(result.email, "valid@example.com");
        assert_eq!(result.status, crate::types::VerificationStatus::Valid);
        assert!(result.is_valid);
    }

    #[tokio::test]
    async fn test_verify_invalid_email() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/email-verification/single"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "email": "invalid@nonexistent.domain",
                    "status": "invalid",
                    "isValid": false,
                    "isDisposable": false,
                    "isRoleBased": false,
                    "isFreeProvider": false,
                    "mxFound": false
                }
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
            .and(path("/api/v1/email-verification/single"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "email": "user@gmial.com",
                    "status": "invalid",
                    "isValid": false,
                    "isDisposable": false,
                    "isRoleBased": false,
                    "isFreeProvider": false,
                    "mxFound": false,
                    "suggestion": "user@gmail.com"
                }
            })))
            .mount(&mock_server)
            .await;

        let result = verification.verify("user@gmial.com").await.unwrap();
        assert_eq!(result.suggestion, Some("user@gmail.com".to_string()));
    }

    #[tokio::test]
    async fn test_batch() {
        let (mock_server, verification) = setup().await;

        // API can return synchronous results when all emails are cached
        Mock::given(method("POST"))
            .and(path("/api/v1/email-verification/batch"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "totalEmails": 3,
                    "creditsDeducted": 6,
                    "status": "completed",
                    "results": {
                        "clean": ["email1@example.com"],
                        "dirty": ["email2@example.com"],
                        "unknown": ["email3@example.com"]
                    },
                    "analytics": {
                        "cleanCount": 1,
                        "dirtyCount": 1,
                        "unknownCount": 1,
                        "cleanPercentage": 33.33
                    }
                }
            })))
            .mount(&mock_server)
            .await;

        let emails = vec![
            "email1@example.com".to_string(),
            "email2@example.com".to_string(),
            "email3@example.com".to_string(),
        ];

        let result = verification.batch(emails).await.unwrap();
        assert_eq!(result.status, "completed");
        assert_eq!(result.total_emails, 3);
    }

    #[tokio::test]
    async fn test_get() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/email-verification/batch_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "verificationId": "batch_123",
                    "status": "completed",
                    "totalEmails": 3,
                    "creditsDeducted": 6,
                    "results": {
                        "clean": ["email1@example.com"],
                        "dirty": ["email2@example.com"],
                        "unknown": ["email3@example.com"]
                    },
                    "analytics": {
                        "cleanCount": 1,
                        "dirtyCount": 1,
                        "unknownCount": 1,
                        "cleanPercentage": 33.33
                    },
                    "createdAt": "2024-01-01T00:00:00Z",
                    "completedAt": "2024-01-01T00:01:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let result = verification.get("batch_123").await.unwrap();
        assert_eq!(result.status, "completed");
        assert!(result.results.is_some());
        let results = result.results.unwrap();
        assert_eq!(results.clean.len(), 1);
        assert_eq!(results.dirty.len(), 1);
        assert_eq!(results.unknown.len(), 1);
    }

    #[tokio::test]
    async fn test_verification_stats() {
        let (mock_server, verification) = setup().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/email-verification/stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "totalVerified": 10000,
                    "totalValid": 8500,
                    "totalInvalid": 1000,
                    "totalUnknown": 100,
                    "totalVerifications": 10000,
                    "validPercentage": 85.0
                }
            })))
            .mount(&mock_server)
            .await;

        let stats = verification.stats().await.unwrap();
        assert_eq!(stats.total_verified, 10000);
        assert_eq!(stats.total_valid, 8500);
    }
}
