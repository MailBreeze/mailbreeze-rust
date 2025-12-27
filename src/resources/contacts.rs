use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    Contact, ContactsResponse, CreateContactParams, ListContactsParams, SuppressParams,
    SuppressReason, UpdateContactParams,
};

/// Contacts API resource - scoped to a specific contact list
///
/// All contact operations are performed within the context of a specific list.
/// Get a contacts instance by calling `client.contacts(list_id)`.
///
/// # Example
/// ```rust,no_run
/// use mailbreeze::MailBreeze;
///
/// #[tokio::main]
/// async fn main() -> mailbreeze::Result<()> {
///     let client = MailBreeze::new("your_api_key")?;
///
///     // Get contacts for a specific list
///     let contacts = client.contacts("list_123");
///
///     // Create a contact in this list
///     let contact = contacts.create(&mailbreeze::CreateContactParams {
///         email: "user@example.com".to_string(),
///         first_name: Some("John".to_string()),
///         ..Default::default()
///     }).await?;
///
///     // List contacts in this list
///     let list = contacts.list(&Default::default()).await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Contacts {
    client: HttpClient,
    list_id: String,
}

impl Contacts {
    pub fn new(client: HttpClient, list_id: impl Into<String>) -> Self {
        Self {
            client,
            list_id: list_id.into(),
        }
    }

    /// Build the path for contact operations within this list
    fn path(&self, suffix: &str) -> String {
        format!("/contact-lists/{}/contacts{}", self.list_id, suffix)
    }

    /// Create a new contact in the list
    pub async fn create(&self, params: &CreateContactParams) -> Result<Contact> {
        self.client.post(&self.path(""), params).await
    }

    /// Get a contact by ID
    pub async fn get(&self, id: &str) -> Result<Contact> {
        self.client.get(&self.path(&format!("/{}", id))).await
    }

    /// Update a contact
    pub async fn update(&self, id: &str, params: &UpdateContactParams) -> Result<Contact> {
        self.client
            .put(&self.path(&format!("/{}", id)), params)
            .await
    }

    /// Delete a contact
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client.delete(&self.path(&format!("/{}", id))).await
    }

    /// List contacts in the list with optional filters
    pub async fn list(&self, params: &ListContactsParams) -> Result<ContactsResponse> {
        self.client.get_with_params(&self.path(""), params).await
    }

    /// Suppress a contact
    ///
    /// Suppressed contacts will not receive any emails.
    /// This is different from unsubscribing - suppression is
    /// typically used for bounces, complaints, or manual removal.
    pub async fn suppress(&self, id: &str, reason: SuppressReason) -> Result<()> {
        let params = SuppressParams { reason };
        self.client
            .post_no_response(&self.path(&format!("/{}/suppress", id)), &params)
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
        let contacts = Contacts::new(client, "list_123");
        (mock_server, contacts)
    }

    #[tokio::test]
    async fn test_create_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/contact-lists/list_123/contacts"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "id": "contact_123",
                    "email": "john@example.com",
                    "firstName": "John",
                    "lastName": "Doe",
                    "status": "active",
                    "source": "api",
                    "createdAt": "2024-01-01T00:00:00Z",
                    "updatedAt": "2024-01-01T00:00:00Z"
                }
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
            .and(path("/api/v1/contact-lists/list_123/contacts/contact_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "id": "contact_123",
                    "email": "john@example.com",
                    "status": "active",
                    "source": "api",
                    "createdAt": "2024-01-01T00:00:00Z",
                    "updatedAt": "2024-01-01T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let contact = contacts.get("contact_123").await.unwrap();
        assert_eq!(contact.id, "contact_123");
    }

    #[tokio::test]
    async fn test_update_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/contact-lists/list_123/contacts/contact_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "id": "contact_123",
                    "email": "john@example.com",
                    "firstName": "Johnny",
                    "status": "active",
                    "source": "api",
                    "createdAt": "2024-01-01T00:00:00Z",
                    "updatedAt": "2024-01-02T00:00:00Z"
                }
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
            .and(path("/api/v1/contact-lists/list_123/contacts/contact_123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        contacts.delete("contact_123").await.unwrap();
    }

    #[tokio::test]
    async fn test_list_contacts() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/contact-lists/list_123/contacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "contacts": [
                        {"id": "contact_1", "email": "a@example.com", "status": "active", "source": "api", "createdAt": "2024-01-01T00:00:00Z", "updatedAt": "2024-01-01T00:00:00Z"},
                        {"id": "contact_2", "email": "b@example.com", "status": "active", "source": "api", "createdAt": "2024-01-01T00:00:00Z", "updatedAt": "2024-01-01T00:00:00Z"}
                    ],
                    "pagination": {"page": 1, "limit": 10, "total": 2, "totalPages": 1, "hasNext": false, "hasPrev": false}
                }
            })))
            .mount(&mock_server)
            .await;

        let result = contacts.list(&ListContactsParams::default()).await.unwrap();
        assert_eq!(result.contacts.len(), 2);
    }

    #[tokio::test]
    async fn test_suppress_contact() {
        let (mock_server, contacts) = setup().await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v1/contact-lists/list_123/contacts/contact_123/suppress",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        contacts
            .suppress("contact_123", SuppressReason::Manual)
            .await
            .unwrap();
    }
}
