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
//!     let email = client.emails.send(&SendEmailParams {
//!         from: "sender@yourdomain.com".to_string(),
//!         to: vec!["recipient@example.com".to_string()],
//!         subject: Some("Hello from MailBreeze!".to_string()),
//!         html: Some("<h1>Welcome!</h1>".to_string()),
//!         ..Default::default()
//!     }).await?;
//!
//!     println!("Email sent with ID: {}", email.id);
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
    /// Contacts API resource
    pub contacts: Contacts,
    /// Lists API resource
    pub lists: Lists,
    /// Verification API resource
    pub verification: Verification,
    /// Attachments API resource
    pub attachments: Attachments,
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
            contacts: Contacts::new(http_client.clone()),
            lists: Lists::new(http_client.clone()),
            verification: Verification::new(http_client.clone()),
            attachments: Attachments::new(http_client),
        })
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

        let email = client.emails.send(&params).await.unwrap();
        assert_eq!(email.id, "email_123");
    }
}
