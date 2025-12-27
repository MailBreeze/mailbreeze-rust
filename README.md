# MailBreeze Rust SDK

Official Rust SDK for [MailBreeze](https://mailbreeze.com) - Email Marketing & Transactional Email Platform.

[![Crates.io](https://img.shields.io/crates/v/mailbreeze.svg)](https://crates.io/crates/mailbreeze)
[![Documentation](https://docs.rs/mailbreeze/badge.svg)](https://docs.rs/mailbreeze)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 📧 **Email Sending** - Transactional and marketing emails
- 👥 **Contact Management** - Create, update, and organize contacts
- 📋 **List Management** - Mailing lists with statistics
- ✅ **Email Verification** - Single and batch verification
- 📎 **Attachments** - Upload and manage email attachments
- 🔄 **Automatic Retries** - Exponential backoff for transient errors
- 🔒 **Secure** - API key redacted from debug output

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mailbreeze = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use mailbreeze::{MailBreeze, SendEmailParams};

#[tokio::main]
async fn main() -> mailbreeze::Result<()> {
    let client = MailBreeze::new("your_api_key")?;

    let email = client.emails.send(&SendEmailParams {
        from: "sender@yourdomain.com".to_string(),
        to: vec!["recipient@example.com".to_string()],
        subject: Some("Hello from MailBreeze!".to_string()),
        html: Some("<h1>Welcome!</h1><p>Thanks for signing up.</p>".to_string()),
        ..Default::default()
    }).await?;

    println!("Email sent with ID: {}", email.id);
    Ok(())
}
```

## Configuration

Use the builder pattern for custom configuration:

```rust
use mailbreeze::MailBreeze;
use std::time::Duration;

let client = MailBreeze::builder("your_api_key")
    .base_url("https://custom.api.com")
    .timeout(Duration::from_secs(60))
    .max_retries(5)
    .build()?;
```

## Resources

### Emails

```rust
// Send an email
let email = client.emails.send(&SendEmailParams {
    from: "sender@example.com".to_string(),
    to: vec!["recipient@example.com".to_string()],
    subject: Some("Hello".to_string()),
    html: Some("<p>Hello World</p>".to_string()),
    ..Default::default()
}).await?;

// Get email by ID
let email = client.emails.get("email_id").await?;

// List emails
let emails = client.emails.list(&ListEmailsParams::default()).await?;

// Get statistics
let stats = client.emails.stats().await?;
```

### Contacts

```rust
// Create a contact
let contact = client.contacts.create(&CreateContactParams {
    email: "user@example.com".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Doe".to_string()),
    ..Default::default()
}).await?;

// Update a contact
let contact = client.contacts.update("contact_id", &UpdateContactParams {
    first_name: Some("Jane".to_string()),
    ..Default::default()
}).await?;

// List contacts
let contacts = client.contacts.list(&ListContactsParams::default()).await?;

// Unsubscribe
let result = client.contacts.unsubscribe("contact_id").await?;
```

### Lists

```rust
// Create a list
let list = client.lists.create(&CreateListParams {
    name: "Newsletter".to_string(),
    description: Some("Weekly newsletter subscribers".to_string()),
}).await?;

// Add contact to list
client.lists.add_contact("list_id", "contact_id").await?;

// Get list statistics
let stats = client.lists.stats("list_id").await?;
```

### Email Verification

```rust
// Verify single email
let result = client.verification.verify("test@example.com").await?;

if result.is_valid {
    println!("Email is valid!");
}

// Batch verification
let batch = client.verification.batch(vec![
    "email1@example.com".to_string(),
    "email2@example.com".to_string(),
]).await?;

// Get verification status
let status = client.verification.get("verification_id").await?;

// Get verification stats
let stats = client.verification.stats().await?;
```

### Attachments

```rust
// Create upload URL
let upload = client.attachments.create_upload_url(&CreateUploadParams {
    filename: "document.pdf".to_string(),
    content_type: "application/pdf".to_string(),
    size: 1024000,
}).await?;

// Use upload_url to upload file, then reference attachment_id in emails
```

## Error Handling

```rust
use mailbreeze::{Error, MailBreeze};

match client.emails.get("invalid_id").await {
    Ok(email) => println!("Found: {}", email.id),
    Err(Error::NotFound { message, .. }) => println!("Email not found: {}", message),
    Err(Error::Authentication { .. }) => println!("Invalid API key"),
    Err(Error::RateLimit { retry_after, .. }) => {
        println!("Rate limited, retry after {:?} seconds", retry_after);
    }
    Err(Error::Validation { errors, .. }) => {
        for (field, messages) in errors {
            println!("{}: {:?}", field, messages);
        }
    }
    Err(e) => println!("Error: {}", e),
}
```

## License

MIT License - see [LICENSE](LICENSE) for details.
