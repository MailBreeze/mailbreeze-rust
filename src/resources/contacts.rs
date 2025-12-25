use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    Contact, ContactList, CreateContactParams, ListContactsParams, SubscriptionResult,
    UpdateContactParams,
};

/// Contacts API resource
#[derive(Debug, Clone)]
pub struct Contacts {
    client: HttpClient,
}

impl Contacts {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Create a new contact
    pub async fn create(&self, params: &CreateContactParams) -> Result<Contact> {
        self.client.post("/contacts", params).await
    }

    /// Get a contact by ID
    pub async fn get(&self, id: &str) -> Result<Contact> {
        self.client.get(&format!("/contacts/{}", id)).await
    }

    /// Update a contact
    pub async fn update(&self, id: &str, params: &UpdateContactParams) -> Result<Contact> {
        self.client
            .patch(&format!("/contacts/{}", id), params)
            .await
    }

    /// Delete a contact
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client.delete(&format!("/contacts/{}", id)).await
    }

    /// List contacts with optional filters
    pub async fn list(&self, params: &ListContactsParams) -> Result<ContactList> {
        self.client.get_with_params("/contacts", params).await
    }

    /// Unsubscribe a contact
    pub async fn unsubscribe(&self, id: &str) -> Result<SubscriptionResult> {
        self.client
            .post_empty(&format!("/contacts/{}/unsubscribe", id))
            .await
    }

    /// Resubscribe a contact
    pub async fn resubscribe(&self, id: &str) -> Result<SubscriptionResult> {
        self.client
            .post_empty(&format!("/contacts/{}/resubscribe", id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use crate::types::ContactStatus;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, Contacts) {
        let mock_server = MockServer::start().await;
        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();
        let contacts = Contacts::new(client);
        (mock_server, contacts)
    }

    #[tokio::test]
    async fn test_create_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("POST"))
            .and(path("/contacts"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "contact_123",
                "email": "john@example.com",
                "first_name": "John",
                "last_name": "Doe",
                "status": "active",
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = CreateContactParams {
            email: "john@example.com".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            ..Default::default()
        };

        let contact = contacts.create(&params).await.unwrap();
        assert_eq!(contact.id, "contact_123");
        assert_eq!(contact.status, ContactStatus::Active);
    }

    #[tokio::test]
    async fn test_get_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("GET"))
            .and(path("/contacts/contact_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "contact_123",
                "email": "john@example.com",
                "status": "active",
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let contact = contacts.get("contact_123").await.unwrap();
        assert_eq!(contact.id, "contact_123");
    }

    #[tokio::test]
    async fn test_update_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("PATCH"))
            .and(path("/contacts/contact_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "contact_123",
                "email": "john@example.com",
                "first_name": "Johnny",
                "status": "active",
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateContactParams {
            first_name: Some("Johnny".to_string()),
            ..Default::default()
        };

        let contact = contacts.update("contact_123", &params).await.unwrap();
        assert_eq!(contact.first_name, Some("Johnny".to_string()));
    }

    #[tokio::test]
    async fn test_delete_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/contacts/contact_123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        contacts.delete("contact_123").await.unwrap();
    }

    #[tokio::test]
    async fn test_list_contacts() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("GET"))
            .and(path("/contacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {"id": "contact_1", "email": "a@example.com", "status": "active", "created_at": "2024-01-01T00:00:00Z"},
                    {"id": "contact_2", "email": "b@example.com", "status": "active", "created_at": "2024-01-01T00:00:00Z"}
                ],
                "meta": {"page": 1, "limit": 10, "total": 2, "total_pages": 1}
            })))
            .mount(&mock_server)
            .await;

        let result = contacts.list(&ListContactsParams::default()).await.unwrap();
        assert_eq!(result.items.len(), 2);
    }

    #[tokio::test]
    async fn test_unsubscribe_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("POST"))
            .and(path("/contacts/contact_123/unsubscribe"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "contact_123",
                "status": "unsubscribed"
            })))
            .mount(&mock_server)
            .await;

        let result = contacts.unsubscribe("contact_123").await.unwrap();
        assert_eq!(result.status, ContactStatus::Unsubscribed);
    }

    #[tokio::test]
    async fn test_resubscribe_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("POST"))
            .and(path("/contacts/contact_123/resubscribe"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "contact_123",
                "status": "active"
            })))
            .mount(&mock_server)
            .await;

        let result = contacts.resubscribe("contact_123").await.unwrap();
        assert_eq!(result.status, ContactStatus::Active);
    }
}
