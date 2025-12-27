//! # MailBreeze
//!
//! Official Rust SDK for MailBreeze - Email Marketing & Transactional Email Platform.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use mailbreeze::{MailBreeze, SendEmailParams};
//!
//! #[tokio::main]
//! async fn main() -> mailbreeze::Result<()> {
//!     let client = MailBreeze::new("your_api_key")?;
//!
//!     // Send an email
//!     let result = client.emails.send(&SendEmailParams {
//!         from: "sender@yourdomain.com".to_string(),
//!         to: vec!["recipient@example.com".to_string()],
//!         subject: Some("Hello from MailBreeze!".to_string()),
//!         html: Some("<h1>Welcome!</h1>".to_string()),
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("Email sent with message ID: {}", result.message_id);
//!
//!     // Work with contacts in a specific list
//!     let contacts = client.contacts("list_xxx");
//!     let contact_list = contacts.list(&Default::default()).await?;
//!
//!     Ok(())
//! }
//! ```

mod client;
mod error;
mod resources;
mod types;

pub use client::{ClientConfig, HttpClient};
pub use error::{Error, Result};
pub use resources::{Attachments, Contacts, Emails, Lists, Verification};
pub use types::*;

use std::time::Duration;

/// Main MailBreeze client
#[derive(Debug, Clone)]
pub struct MailBreeze {
    /// Emails API resource
    pub emails: Emails,
    /// Lists API resource
    pub lists: Lists,
    /// Verification API resource
    pub verification: Verification,
    /// Attachments API resource
    pub attachments: Attachments,
    /// HTTP client for creating list-scoped resources
    http_client: HttpClient,
}

impl MailBreeze {
    /// Create a new MailBreeze client with the given API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::with_config(ClientConfig::new(api_key))
    }

    /// Create a new MailBreeze client with custom configuration
    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let http_client = HttpClient::new(config)?;

        Ok(Self {
            emails: Emails::new(http_client.clone()),
            lists: Lists::new(http_client.clone()),
            verification: Verification::new(http_client.clone()),
            attachments: Attachments::new(http_client.clone()),
            http_client,
        })
    }

    /// Get a contacts resource for a specific list
    ///
    /// All contact operations are performed within the context of a specific list.
    ///
    /// # Arguments
    /// * `list_id` - The ID of the contact list
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
    ///         ..Default::default()
    ///     }).await?;
    ///
    ///     // List contacts in this list
    ///     let list = contacts.list(&Default::default()).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn contacts(&self, list_id: impl Into<String>) -> Contacts {
        Contacts::new(self.http_client.clone(), list_id)
    }

    /// Create a builder for configuring the client
    pub fn builder(api_key: impl Into<String>) -> MailBreezeBuilder {
        MailBreezeBuilder::new(api_key)
    }
}

/// Builder for creating a MailBreeze client with custom configuration
pub struct MailBreezeBuilder {
    config: ClientConfig,
}

impl MailBreezeBuilder {
    /// Create a new builder with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            config: ClientConfig::new(api_key),
        }
    }

    /// Set a custom base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config = self.config.base_url(url);
        self
    }

    /// Set the request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config = self.config.timeout(timeout);
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config = self.config.max_retries(retries);
        self
    }

    /// Build the MailBreeze client
    pub fn build(self) -> Result<MailBreeze> {
        MailBreeze::with_config(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_client_creation() {
        let client = MailBreeze::new("test_api_key");
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_builder_pattern() {
        let client = MailBreeze::builder("test_api_key")
            .base_url("https://custom.api.com")
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .build();

        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_send_email_integration() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/emails"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "messageId": "msg_123abc"
                }
            })))
            .mount(&mock_server)
            .await;

        let client = MailBreeze::builder("test_api_key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();

        let params = SendEmailParams {
            from: "sender@example.com".to_string(),
            to: vec!["recipient@example.com".to_string()],
            subject: Some("Hello".to_string()),
            html: Some("<p>Test</p>".to_string()),
            ..Default::default()
        };

        let result = client.emails.send(&params).await.unwrap();
        assert_eq!(result.message_id, "msg_123abc");
    }

    #[tokio::test]
    async fn test_contacts_method() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/contact-lists/list_123/contacts"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "success": true,
                "data": {
                    "contacts": [
                        {"id": "contact_1", "email": "a@example.com", "status": "active", "source": "api", "createdAt": "2024-01-01T00:00:00Z", "updatedAt": "2024-01-01T00:00:00Z"}
                    ],
                    "pagination": {"page": 1, "limit": 10, "total": 1, "totalPages": 1, "hasNext": false, "hasPrev": false}
                }
            })))
            .mount(&mock_server)
            .await;

        let client = MailBreeze::builder("test_api_key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();

        let contacts = client.contacts("list_123");
        let result = contacts.list(&ListContactsParams::default()).await.unwrap();
        assert_eq!(result.contacts.len(), 1);
    }
}
