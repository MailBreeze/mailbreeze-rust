use crate::error::{Error, Result};
use reqwest::{Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.mailbreeze.com/v1";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 3;

/// Configuration for the MailBreeze client
#[derive(Clone)]
pub struct ClientConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

// Custom Debug implementation that redacts the API key
impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("api_key", &"[REDACTED]")
            .field("base_url", &self.base_url)
            .field("timeout", &self.timeout)
            .field("max_retries", &self.max_retries)
            .finish()
    }
}

impl ClientConfig {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            max_retries: DEFAULT_MAX_RETRIES,
        }
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }
}

/// HTTP client for MailBreeze API
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    config: ClientConfig,
}

impl HttpClient {
    /// Create a new HTTP client with the given configuration
    pub fn new(config: ClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(Error::Http)?;

        Ok(Self { client, config })
    }

    /// Perform a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request_impl(Method::GET, path, None, None).await
    }

    /// Perform a GET request with query parameters
    pub async fn get_with_params<T, Q>(&self, path: &str, params: &Q) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize,
    {
        let query = serde_json::to_value(params).ok();
        self.request_impl(Method::GET, path, None, query.as_ref())
            .await
    }

    /// Perform a POST request
    pub async fn post<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let body_value = serde_json::to_value(body)?;
        self.request_impl(Method::POST, path, Some(&body_value), None)
            .await
    }

    /// Perform a POST request without a body
    pub async fn post_empty<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request_impl(Method::POST, path, None, None).await
    }

    /// Perform a PATCH request
    pub async fn patch<T, B>(&self, path: &str, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let body_value = serde_json::to_value(body)?;
        self.request_impl(Method::PATCH, path, Some(&body_value), None)
            .await
    }

    /// Perform a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        self.request_no_response(Method::DELETE, path).await
    }

    /// Internal request implementation
    async fn request_impl<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&serde_json::Value>,
        query: Option<&serde_json::Value>,
    ) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut attempt = 0;

        loop {
            attempt += 1;

            let mut request = self.client.request(method.clone(), &url);
            request = request
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("User-Agent", "mailbreeze-rust/0.1.0");

            if let Some(b) = body {
                request = request.json(b);
            }

            if let Some(q) = query {
                if let Some(obj) = q.as_object() {
                    for (key, value) in obj {
                        if let Some(s) = value.as_str() {
                            request = request.query(&[(key, s)]);
                        } else if !value.is_null() {
                            request = request.query(&[(key, value.to_string())]);
                        }
                    }
                }
            }

            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    if attempt < self.config.max_retries && (e.is_connect() || e.is_timeout()) {
                        self.wait_before_retry(attempt).await;
                        continue;
                    }
                    return Err(Error::Http(e));
                }
            };

            match self.handle_response(response).await {
                Ok(data) => return Ok(data),
                Err(e) if e.is_retryable() && attempt < self.config.max_retries => {
                    self.wait_before_retry(attempt).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }

    /// Perform a request that expects no response body
    async fn request_no_response(&self, method: Method, path: &str) -> Result<()> {
        let url = format!("{}{}", self.config.base_url, path);
        let mut attempt = 0;

        loop {
            attempt += 1;

            let request = self
                .client
                .request(method.clone(), &url)
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("User-Agent", "mailbreeze-rust/0.1.0");

            let response = match request.send().await {
                Ok(resp) => resp,
                Err(e) => {
                    if attempt < self.config.max_retries && (e.is_connect() || e.is_timeout()) {
                        self.wait_before_retry(attempt).await;
                        continue;
                    }
                    return Err(Error::Http(e));
                }
            };

            let status = response.status();
            if status == StatusCode::NO_CONTENT || status.is_success() {
                return Ok(());
            }

            let error = self.parse_error_response(response).await?;
            if error.is_retryable() && attempt < self.config.max_retries {
                self.wait_before_retry(attempt).await;
                continue;
            }
            return Err(error);
        }
    }

    /// Handle the response and parse JSON or error
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            let text = response.text().await.map_err(Error::Http)?;
            if text.is_empty() {
                return Err(Error::Json(serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Empty response body",
                ))));
            }
            serde_json::from_str(&text).map_err(Error::Json)
        } else {
            Err(self.parse_error_response(response).await?)
        }
    }

    /// Parse an error response
    async fn parse_error_response(&self, response: Response) -> Result<Error> {
        let status = response.status();
        let retry_after = response
            .headers()
            .get("Retry-After")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| self.parse_retry_after(v));

        let body: HashMap<String, serde_json::Value> = response.json().await.unwrap_or_default();

        let message = body
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown error")
            .to_string();

        let code = body
            .get("code")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let error = match status {
            StatusCode::BAD_REQUEST => Error::BadRequest { message, code },
            StatusCode::UNAUTHORIZED => Error::Authentication { message, code },
            StatusCode::NOT_FOUND => Error::NotFound { message, code },
            StatusCode::UNPROCESSABLE_ENTITY => {
                let errors = body
                    .get("errors")
                    .and_then(|v| serde_json::from_value(v.clone()).ok())
                    .unwrap_or_default();
                Error::Validation {
                    message,
                    errors,
                    code,
                }
            }
            StatusCode::TOO_MANY_REQUESTS => Error::RateLimit {
                message,
                retry_after,
                code,
            },
            _ if status.is_server_error() => Error::Server {
                message,
                status_code: status.as_u16(),
                code,
            },
            _ => Error::Server {
                message,
                status_code: status.as_u16(),
                code,
            },
        };

        Ok(error)
    }

    /// Parse Retry-After header (integer seconds or HTTP-date)
    fn parse_retry_after(&self, value: &str) -> Option<u64> {
        // Try parsing as integer seconds
        if let Ok(seconds) = value.parse::<u64>() {
            return Some(seconds);
        }

        // Try parsing as HTTP-date (RFC 1123)
        if let Ok(date) = chrono::DateTime::parse_from_rfc2822(value) {
            let now = chrono::Utc::now();
            let delta = date.signed_duration_since(now);
            if delta.num_seconds() > 0 {
                return Some(delta.num_seconds() as u64);
            }
        }

        None
    }

    /// Wait before retrying with exponential backoff
    async fn wait_before_retry(&self, attempt: u32) {
        let delay = Duration::from_millis(100 * (1 << (attempt - 1)));
        tokio::time::sleep(delay).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_successful_get_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .and(header("Authorization", "Bearer test_key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": "123",
                "name": "Test"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();

        let result: serde_json::Value = client.get("/test").await.unwrap();
        assert_eq!(result["id"], "123");
        assert_eq!(result["name"], "Test");
    }

    #[tokio::test]
    async fn test_successful_post_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": "456"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();

        let body = serde_json::json!({"name": "Test"});
        let result: serde_json::Value = client.post("/test", &body).await.unwrap();
        assert_eq!(result["id"], "456");
    }

    #[tokio::test]
    async fn test_delete_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/test/123"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();

        client.delete("/test/123").await.unwrap();
    }

    #[tokio::test]
    async fn test_authentication_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
                "error": "Invalid API key"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("bad_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();

        let result: std::result::Result<serde_json::Value, _> = client.get("/test").await;
        assert!(matches!(result, Err(Error::Authentication { .. })));
    }

    #[tokio::test]
    async fn test_not_found_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test/nonexistent"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "error": "Not found"
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();

        let result: std::result::Result<serde_json::Value, _> =
            client.get("/test/nonexistent").await;
        assert!(matches!(result, Err(Error::NotFound { .. })));
    }

    #[tokio::test]
    async fn test_validation_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "error": "Validation failed",
                "errors": {
                    "email": ["Required"]
                }
            })))
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key").base_url(mock_server.uri());
        let client = HttpClient::new(config).unwrap();

        let body = serde_json::json!({});
        let result: std::result::Result<serde_json::Value, _> = client.post("/test", &body).await;

        match result {
            Err(Error::Validation { errors, .. }) => {
                assert!(errors.contains_key("email"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "30")
                    .set_body_json(serde_json::json!({
                        "error": "Rate limit exceeded"
                    })),
            )
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key")
            .base_url(mock_server.uri())
            .max_retries(1);
        let client = HttpClient::new(config).unwrap();

        let result: std::result::Result<serde_json::Value, _> = client.get("/test").await;

        match result {
            Err(Error::RateLimit { retry_after, .. }) => {
                assert_eq!(retry_after, Some(30));
            }
            _ => panic!("Expected rate limit error"),
        }
    }

    #[tokio::test]
    async fn test_retry_on_server_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "error": "Server error"
            })))
            .expect(3)
            .mount(&mock_server)
            .await;

        let config = ClientConfig::new("test_key")
            .base_url(mock_server.uri())
            .max_retries(3);
        let client = HttpClient::new(config).unwrap();

        let result: std::result::Result<serde_json::Value, _> = client.get("/test").await;
        assert!(matches!(result, Err(Error::Server { .. })));
    }

    #[test]
    fn test_api_key_redacted_in_debug() {
        let config = ClientConfig::new("super_secret_api_key_12345");
        let debug_output = format!("{:?}", config);

        // API key should NOT appear in debug output
        assert!(!debug_output.contains("super_secret_api_key_12345"));
        // Should show [REDACTED] instead
        assert!(debug_output.contains("[REDACTED]"));
    }
}
