use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    CancelEnrollmentResult, EnrollParams, Enrollment, EnrollmentList, ListEnrollmentsParams,
};

/// Automations API resource
#[derive(Debug, Clone)]
pub struct Automations {
    client: HttpClient,
}

impl Automations {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Enroll a contact in an automation
    pub async fn enroll(&self, params: &EnrollParams) -> Result<Enrollment> {
        self.client.post("/automations/enrollments", params).await
    }

    /// Get an enrollment by ID
    pub async fn get_enrollment(&self, id: &str) -> Result<Enrollment> {
        self.client
            .get(&format!("/automations/enrollments/{}", id))
            .await
    }

    /// List enrollments with optional filters
    pub async fn list_enrollments(&self, params: &ListEnrollmentsParams) -> Result<EnrollmentList> {
        self.client
            .get_with_params("/automations/enrollments", params)
            .await
    }

    /// Cancel an enrollment
    pub async fn cancel_enrollment(&self, id: &str) -> Result<CancelEnrollmentResult> {
        self.client
            .post_empty(&format!("/automations/enrollments/{}/cancel", id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use crate::types::EnrollmentStatus;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, Automations) {
        let mock_server = MockServer::start().await;
        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();
        let automations = Automations::new(client);
        (mock_server, automations)
    }

    #[tokio::test]
    async fn test_enroll_contact() {
        let (mock_server, automations) = setup().await;

        Mock::given(method("POST"))
            .and(path("/automations/enrollments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "enrollment_123",
                "automation_id": "auto_456",
                "contact_id": "contact_789",
                "status": "active",
                "current_step": 0,
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = EnrollParams {
            automation_id: "auto_456".to_string(),
            contact_id: "contact_789".to_string(),
            variables: None,
        };

        let enrollment = automations.enroll(&params).await.unwrap();
        assert_eq!(enrollment.id, "enrollment_123");
        assert_eq!(enrollment.status, EnrollmentStatus::Active);
    }

    #[tokio::test]
    async fn test_enroll_with_variables() {
        let (mock_server, automations) = setup().await;

        Mock::given(method("POST"))
            .and(path("/automations/enrollments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "enrollment_456",
                "automation_id": "auto_456",
                "contact_id": "contact_789",
                "status": "active",
                "current_step": 0,
                "variables": {
                    "discount_code": "WELCOME10",
                    "trial_days": 14
                },
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let mut variables = std::collections::HashMap::new();
        variables.insert("discount_code".to_string(), serde_json::json!("WELCOME10"));
        variables.insert("trial_days".to_string(), serde_json::json!(14));

        let params = EnrollParams {
            automation_id: "auto_456".to_string(),
            contact_id: "contact_789".to_string(),
            variables: Some(variables),
        };

        let enrollment = automations.enroll(&params).await.unwrap();
        assert!(enrollment.variables.is_some());
    }

    #[tokio::test]
    async fn test_get_enrollment() {
        let (mock_server, automations) = setup().await;

        Mock::given(method("GET"))
            .and(path("/automations/enrollments/enrollment_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "enrollment_123",
                "automation_id": "auto_456",
                "contact_id": "contact_789",
                "status": "active",
                "current_step": 2,
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let enrollment = automations.get_enrollment("enrollment_123").await.unwrap();
        assert_eq!(enrollment.current_step, 2);
    }

    #[tokio::test]
    async fn test_list_enrollments() {
        let (mock_server, automations) = setup().await;

        Mock::given(method("GET"))
            .and(path("/automations/enrollments"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {"id": "enrollment_1", "automation_id": "auto_1", "contact_id": "contact_1", "status": "active", "current_step": 0, "created_at": "2024-01-01T00:00:00Z"},
                    {"id": "enrollment_2", "automation_id": "auto_1", "contact_id": "contact_2", "status": "completed", "current_step": 5, "created_at": "2024-01-01T00:00:00Z"}
                ],
                "meta": {"page": 1, "limit": 10, "total": 2, "total_pages": 1}
            })))
            .mount(&mock_server)
            .await;

        let result = automations
            .list_enrollments(&ListEnrollmentsParams::default())
            .await
            .unwrap();
        assert_eq!(result.items.len(), 2);
    }

    #[tokio::test]
    async fn test_cancel_enrollment() {
        let (mock_server, automations) = setup().await;

        Mock::given(method("POST"))
            .and(path("/automations/enrollments/enrollment_123/cancel"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "enrollment_123",
                "cancelled": true
            })))
            .mount(&mock_server)
            .await;

        let result = automations
            .cancel_enrollment("enrollment_123")
            .await
            .unwrap();
        assert!(result.cancelled);
    }
}
