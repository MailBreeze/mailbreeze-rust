use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    AddContactResult, ContactList, CreateListParams, List, ListListsParams, ListStats,
    ListsResponse, RemoveContactResult, UpdateListParams,
};

/// Lists API resource
#[derive(Debug, Clone)]
pub struct Lists {
    client: HttpClient,
}

impl Lists {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Create a new list
    pub async fn create(&self, params: &CreateListParams) -> Result<List> {
        self.client.post("/lists", params).await
    }

    /// Get a list by ID
    pub async fn get(&self, id: &str) -> Result<List> {
        self.client.get(&format!("/lists/{}", id)).await
    }

    /// Update a list
    pub async fn update(&self, id: &str, params: &UpdateListParams) -> Result<List> {
        self.client.patch(&format!("/lists/{}", id), params).await
    }

    /// Delete a list
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client.delete(&format!("/lists/{}", id)).await
    }

    /// List all lists with optional filters
    pub async fn list(&self, params: &ListListsParams) -> Result<ListsResponse> {
        self.client.get_with_params("/lists", params).await
    }

    /// Get list statistics
    pub async fn stats(&self, id: &str) -> Result<ListStats> {
        self.client.get(&format!("/lists/{}/stats", id)).await
    }

    /// Add a contact to a list
    pub async fn add_contact(&self, list_id: &str, contact_id: &str) -> Result<AddContactResult> {
        self.client
            .post_empty(&format!("/lists/{}/contacts/{}", list_id, contact_id))
            .await
    }

    /// Remove a contact from a list
    pub async fn remove_contact(
        &self,
        list_id: &str,
        contact_id: &str,
    ) -> Result<RemoveContactResult> {
        // DELETE returns JSON for this endpoint
        self.client
            .post_empty(&format!(
                "/lists/{}/contacts/{}/remove",
                list_id, contact_id
            ))
            .await
    }

    /// List contacts in a list
    pub async fn contacts(&self, list_id: &str) -> Result<ContactList> {
        self.client
            .get(&format!("/lists/{}/contacts", list_id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::ClientConfig;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup() -> (MockServer, Lists) {
        let mock_server = MockServer::start().await;
        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();
        let lists = Lists::new(client);
        (mock_server, lists)
    }

    #[tokio::test]
    async fn test_create_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("POST"))
            .and(path("/lists"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "list_123",
                "name": "Newsletter",
                "description": "Weekly newsletter",
                "contact_count": 0,
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = CreateListParams {
            name: "Newsletter".to_string(),
            description: Some("Weekly newsletter".to_string()),
        };

        let list = lists.create(&params).await.unwrap();
        assert_eq!(list.id, "list_123");
        assert_eq!(list.name, "Newsletter");
    }

    #[tokio::test]
    async fn test_get_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("GET"))
            .and(path("/lists/list_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "list_123",
                "name": "Newsletter",
                "contact_count": 100,
                "created_at": "2024-01-01T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let list = lists.get("list_123").await.unwrap();
        assert_eq!(list.contact_count, 100);
    }

    #[tokio::test]
    async fn test_update_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("PATCH"))
            .and(path("/lists/list_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "list_123",
                "name": "Updated Newsletter",
                "contact_count": 100,
                "created_at": "2024-01-01T00:00:00Z",
                "updated_at": "2024-01-02T00:00:00Z"
            })))
            .mount(&mock_server)
            .await;

        let params = UpdateListParams {
            name: Some("Updated Newsletter".to_string()),
            ..Default::default()
        };

        let list = lists.update("list_123", &params).await.unwrap();
        assert_eq!(list.name, "Updated Newsletter");
    }

    #[tokio::test]
    async fn test_delete_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("DELETE"))
            .and(path("/lists/list_123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        lists.delete("list_123").await.unwrap();
    }

    #[tokio::test]
    async fn test_list_lists() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("GET"))
            .and(path("/lists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {"id": "list_1", "name": "List A", "contact_count": 50, "created_at": "2024-01-01T00:00:00Z"},
                    {"id": "list_2", "name": "List B", "contact_count": 100, "created_at": "2024-01-01T00:00:00Z"}
                ],
                "meta": {"page": 1, "limit": 10, "total": 2, "total_pages": 1}
            })))
            .mount(&mock_server)
            .await;

        let result = lists.list(&ListListsParams::default()).await.unwrap();
        assert_eq!(result.items.len(), 2);
    }

    #[tokio::test]
    async fn test_get_list_stats() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("GET"))
            .and(path("/lists/list_123/stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total": 1000,
                "active": 900,
                "unsubscribed": 80,
                "bounced": 15,
                "complained": 5
            })))
            .mount(&mock_server)
            .await;

        let stats = lists.stats("list_123").await.unwrap();
        assert_eq!(stats.total, 1000);
        assert_eq!(stats.active, 900);
    }

    #[tokio::test]
    async fn test_add_contact_to_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("POST"))
            .and(path("/lists/list_123/contacts/contact_456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "added": true
            })))
            .mount(&mock_server)
            .await;

        let result = lists.add_contact("list_123", "contact_456").await.unwrap();
        assert!(result.added);
    }

    #[tokio::test]
    async fn test_list_contacts_in_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("GET"))
            .and(path("/lists/list_123/contacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    {"id": "contact_1", "email": "a@example.com", "status": "active", "created_at": "2024-01-01T00:00:00Z"}
                ],
                "meta": {"page": 1, "limit": 10, "total": 1, "total_pages": 1}
            })))
            .mount(&mock_server)
            .await;

        let result = lists.contacts("list_123").await.unwrap();
        assert_eq!(result.items.len(), 1);
    }
}
