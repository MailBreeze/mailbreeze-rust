use crate::client::HttpClient;
use crate::error::Result;
use crate::types::{
    CreateListParams, List, ListListsParams, ListStats, ListsResponse, Pagination, UpdateListParams,
};

/// Contact lists API resource
///
/// Manage contact lists in your MailBreeze account.
///
/// # Example
/// ```rust,no_run
/// use mailbreeze::MailBreeze;
///
/// #[tokio::main]
/// async fn main() -> mailbreeze::Result<()> {
///     let client = MailBreeze::new("your_api_key")?;
///
///     // Create a new list
///     let list = client.lists.create(&mailbreeze::CreateListParams {
///         name: "Newsletter".to_string(),
///         description: Some("Weekly newsletter subscribers".to_string()),
///     }).await?;
///
///     // Get contacts for this list
///     let contacts = client.contacts(&list.id);
///     let contact_list = contacts.list(&Default::default()).await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Lists {
    client: HttpClient,
}

impl Lists {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// Create a new contact list
    pub async fn create(&self, params: &CreateListParams) -> Result<List> {
        self.client.post("/contact-lists", params).await
    }

    /// Get a contact list by ID
    pub async fn get(&self, id: &str) -> Result<List> {
        self.client.get(&format!("/contact-lists/{}", id)).await
    }

    /// Update a contact list
    pub async fn update(&self, id: &str, params: &UpdateListParams) -> Result<List> {
        self.client
            .put(&format!("/contact-lists/{}", id), params)
            .await
    }

    /// Delete a contact list
    pub async fn delete(&self, id: &str) -> Result<()> {
        self.client.delete(&format!("/contact-lists/{}", id)).await
    }

    /// List all contact lists with optional filters
    ///
    /// Note: The current API returns a flat array. Pagination may be added in future versions.
    pub async fn list(&self, params: &ListListsParams) -> Result<ListsResponse> {
        // API returns data as a direct array, not wrapped in {lists: [...]}
        let lists: Vec<List> = self
            .client
            .get_with_params("/contact-lists", params)
            .await?;
        Ok(ListsResponse {
            lists,
            pagination: Pagination {
                page: params.page.unwrap_or(1),
                limit: params.limit.unwrap_or(50),
                total: 0, // API doesn't return total count currently
                total_pages: 0,
                has_next: false,
                has_prev: false,
            },
        })
    }

    /// Get contact list statistics
    pub async fn stats(&self, id: &str) -> Result<ListStats> {
        self.client
            .get(&format!("/contact-lists/{}/stats", id))
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
            .and(path("/api/v1/contact-lists"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "id": "list_123",
                    "name": "Newsletter",
                    "description": "Weekly newsletter",
                    "totalContacts": 0,
                    "activeContacts": 0,
                    "suppressedContacts": 0,
                    "tags": [],
                    "createdAt": "2024-01-01T00:00:00Z"
                }
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
            .and(path("/api/v1/contact-lists/list_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "id": "list_123",
                    "name": "Newsletter",
                    "totalContacts": 100,
                    "activeContacts": 95,
                    "suppressedContacts": 5,
                    "tags": [],
                    "createdAt": "2024-01-01T00:00:00Z"
                }
            })))
            .mount(&mock_server)
            .await;

        let list = lists.get("list_123").await.unwrap();
        assert_eq!(list.total_contacts, 100);
    }

    #[tokio::test]
    async fn test_update_list() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/contact-lists/list_123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "id": "list_123",
                    "name": "Updated Newsletter",
                    "totalContacts": 100,
                    "activeContacts": 95,
                    "suppressedContacts": 5,
                    "tags": [],
                    "createdAt": "2024-01-01T00:00:00Z",
                    "updatedAt": "2024-01-02T00:00:00Z"
                }
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
            .and(path("/api/v1/contact-lists/list_123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        lists.delete("list_123").await.unwrap();
    }

    #[tokio::test]
    async fn test_list_lists() {
        let (mock_server, lists) = setup().await;

        // API returns data as a direct array (pagination may be added later)
        Mock::given(method("GET"))
            .and(path("/api/v1/contact-lists"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": [
                    {"id": "list_1", "name": "List A", "totalContacts": 50, "activeContacts": 48, "suppressedContacts": 2, "tags": [], "createdAt": "2024-01-01T00:00:00Z"},
                    {"id": "list_2", "name": "List B", "totalContacts": 100, "activeContacts": 95, "suppressedContacts": 5, "tags": [], "createdAt": "2024-01-01T00:00:00Z"}
                ]
            })))
            .mount(&mock_server)
            .await;

        let result = lists.list(&ListListsParams::default()).await.unwrap();
        assert_eq!(result.lists.len(), 2);
    }

    #[tokio::test]
    async fn test_get_list_stats() {
        let (mock_server, lists) = setup().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/contact-lists/list_123/stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "totalContacts": 1000,
                    "activeContacts": 900,
                    "suppressedContacts": 100
                }
            })))
            .mount(&mock_server)
            .await;

        let stats = lists.stats("list_123").await.unwrap();
        assert_eq!(stats.total_contacts, 1000);
        assert_eq!(stats.active_contacts, 900);
    }
}
