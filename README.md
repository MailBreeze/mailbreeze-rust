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

    let result = client.emails.send(&SendEmailParams {
        from: "sender@yourdomain.com".to_string(),
        to: vec!["recipient@example.com".to_string()],
        subject: Some("Hello from MailBreeze!".to_string()),
        html: Some("<h1>Welcome!</h1><p>Thanks for signing up.</p>".to_string()),
        ..Default::default()
    }).await?;

    println!("Email sent with message ID: {}", result.message_id);
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
let result = client.emails.send(&SendEmailParams {
    from: "sender@example.com".to_string(),
    to: vec!["recipient@example.com".to_string()],
    subject: Some("Hello".to_string()),
    html: Some("<p>Hello World</p>".to_string()),
    ..Default::default()
}).await?;
println!("Sent with ID: {}", result.message_id);

// Get email by ID
let email = client.emails.get("email_id").await?;

// List emails
let email_list = client.emails.list(&ListEmailsParams::default()).await?;
for email in email_list.emails {
    println!("Email: {} - {}", email.id, email.status);
}

// Get statistics
let stats = client.emails.stats().await?;
```

### Contacts

Contacts are managed within a specific list:

```rust
// Get contacts resource for a list
let contacts = client.contacts("list_id");

// Create a contact
let contact = contacts.create(&CreateContactParams {
    email: "user@example.com".to_string(),
    first_name: Some("John".to_string()),
    last_name: Some("Doe".to_string()),
    ..Default::default()
}).await?;

// Update a contact
let contact = contacts.update("contact_id", &UpdateContactParams {
    first_name: Some("Jane".to_string()),
    ..Default::default()
}).await?;

// List contacts
let contact_list = contacts.list(&ListContactsParams::default()).await?;
for contact in contact_list.contacts {
    println!("Contact: {} - {}", contact.email, contact.status);
}

// Suppress a contact (prevent receiving emails)
contacts.suppress("contact_id", "manual").await?;

// Delete a contact
contacts.delete("contact_id").await?;
```

### Lists

```rust
// Create a list
let list = client.lists.create(&CreateListParams {
    name: "Newsletter".to_string(),
    description: Some("Weekly newsletter subscribers".to_string()),
}).await?;

// List all lists
let all_lists = client.lists.list(&ListListsParams::default()).await?;
for list in all_lists.lists {
    println!("List: {} ({} contacts)", list.name, list.total_contacts);
}

// Get a specific list
let list = client.lists.get("list_id").await?;

// Update a list
let list = client.lists.update("list_id", &UpdateListParams {
    name: Some("Updated Name".to_string()),
    ..Default::default()
}).await?;

// Get list statistics
let stats = client.lists.stats("list_id").await?;
println!("Active: {}, Suppressed: {}", stats.active_contacts, stats.suppressed_contacts);

// Delete a list
client.lists.delete("list_id").await?;
```

### Email Verification

```rust
// Verify single email
let result = client.verification.verify("test@example.com").await?;

if result.is_valid {
    println!("Email is valid!");
}

// Batch verification (returns immediate results or verification_id for polling)
let batch = client.verification.batch(vec![
    "email1@example.com".to_string(),
    "email2@example.com".to_string(),
]).await?;

// Results are categorized as clean, dirty, or unknown
if let Some(results) = batch.results {
    println!("Clean: {:?}", results.clean);
    println!("Dirty: {:?}", results.dirty);
    println!("Unknown: {:?}", results.unknown);
}

// Get verification status (for async batches with verification_id)
if !batch.verification_id.is_empty() {
    let status = client.verification.get(&batch.verification_id).await?;
}

// List all verification batches
let batches = client.verification.list().await?;
for batch in batches {
    println!("Batch {}: {} emails, status: {}", batch.id, batch.total_emails, batch.status);
}

// Get verification stats
let stats = client.verification.stats().await?;
println!("Valid: {}%", stats.valid_percentage);
```

### Attachments

```rust
// Step 1: Create upload URL
let upload = client.attachments.create_upload_url(&CreateUploadParams {
    filename: "document.pdf".to_string(),
    content_type: "application/pdf".to_string(),
    size: 1024000,
}).await?;

// Step 2: Upload file to the presigned URL (using reqwest or similar)
// reqwest::Client::new()
//     .put(&upload.upload_url)
//     .body(file_bytes)
//     .header("Content-Type", "application/pdf")
//     .send()
//     .await?;

// Step 3: Confirm the upload
client.attachments.confirm(&upload.attachment_id).await?;

// Step 4: Use in email
let result = client.emails.send(&SendEmailParams {
    from: "sender@example.com".to_string(),
    to: vec!["recipient@example.com".to_string()],
    subject: Some("Your Report".to_string()),
    html: Some("<p>Please find your report attached.</p>".to_string()),
    attachment_ids: Some(vec![upload.attachment_id]),
    ..Default::default()
}).await?;

// Get attachment details
let attachment = client.attachments.get("attachment_id").await?;
println!("Status: {}", attachment.status);

// Delete an attachment
client.attachments.delete("attachment_id").await?;
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
