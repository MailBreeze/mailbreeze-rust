use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pagination metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    pub page: i32,
    pub limit: i32,
    pub total: i32,
    pub total_pages: i32,
}

/// Email delivery status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EmailStatus {
    Pending,
    Queued,
    Sent,
    Delivered,
    Bounced,
    Complained,
    Failed,
}

/// Email object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    pub status: EmailStatus,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_at: Option<String>,
}

/// Parameters for sending an email
#[derive(Debug, Clone, Serialize, Default)]
pub struct SendEmailParams {
    pub from: String,
    pub to: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Parameters for listing emails
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListEmailsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmailStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Paginated list of emails
#[derive(Debug, Clone, Deserialize)]
pub struct EmailList {
    pub items: Vec<Email>,
    pub meta: PaginationMeta,
}

/// Email statistics
#[derive(Debug, Clone, Deserialize)]
pub struct EmailStats {
    pub sent: i64,
    pub delivered: i64,
    pub bounced: i64,
    pub complained: i64,
    pub opened: i64,
    pub clicked: i64,
    pub unsubscribed: i64,
}

/// Contact subscription status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ContactStatus {
    Active,
    Unsubscribed,
    Bounced,
    Complained,
    Suppressed,
}

/// Consent type for NDPR compliance
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConsentType {
    Explicit,
    Implicit,
    LegitimateInterest,
}

/// Contact object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub status: ContactStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_type: Option<ConsentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_ip_address: Option<String>,
}

/// Parameters for creating a contact
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateContactParams {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_type: Option<ConsentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_ip_address: Option<String>,
}

/// Parameters for updating a contact
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateContactParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_type: Option<ConsentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_ip_address: Option<String>,
}

/// Parameters for listing contacts
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListContactsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ContactStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// Paginated list of contacts
#[derive(Debug, Clone, Deserialize)]
pub struct ContactList {
    pub items: Vec<Contact>,
    pub meta: PaginationMeta,
}

/// List (mailing list) object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub contact_count: i32,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Parameters for creating a list
#[derive(Debug, Clone, Serialize)]
pub struct CreateListParams {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameters for updating a list
#[derive(Debug, Clone, Serialize, Default)]
pub struct UpdateListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Parameters for listing lists
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListListsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// Paginated list of lists
#[derive(Debug, Clone, Deserialize)]
pub struct ListsResponse {
    pub items: Vec<List>,
    pub meta: PaginationMeta,
}

/// List statistics
#[derive(Debug, Clone, Deserialize)]
pub struct ListStats {
    pub total: i64,
    pub active: i64,
    pub unsubscribed: i64,
    pub bounced: i64,
    pub complained: i64,
}

/// Verification result status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Valid,
    Invalid,
    Risky,
    Unknown,
}

/// Single email verification result
#[derive(Debug, Clone, Deserialize)]
pub struct VerificationResult {
    pub email: String,
    pub status: VerificationStatus,
    pub is_valid: bool,
    pub is_disposable: bool,
    pub is_role_based: bool,
    pub is_free_provider: bool,
    pub mx_found: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_check: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// Batch verification result
#[derive(Debug, Clone, Deserialize)]
pub struct BatchVerificationResult {
    pub verification_id: String,
    pub status: String,
    pub total: i32,
    pub processed: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<VerificationResult>>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// Verification statistics
#[derive(Debug, Clone, Deserialize)]
pub struct VerificationStats {
    pub total_verified: i64,
    pub valid_count: i64,
    pub invalid_count: i64,
    pub risky_count: i64,
    pub unknown_count: i64,
}

/// Enrollment status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EnrollmentStatus {
    Active,
    Completed,
    Cancelled,
    Failed,
}

/// Automation enrollment
#[derive(Debug, Clone, Deserialize)]
pub struct Enrollment {
    pub id: String,
    pub automation_id: String,
    pub contact_id: String,
    pub status: EnrollmentStatus,
    pub current_step: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, serde_json::Value>>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
}

/// Parameters for enrolling a contact
#[derive(Debug, Clone, Serialize)]
pub struct EnrollParams {
    pub automation_id: String,
    pub contact_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for listing enrollments
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListEnrollmentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EnrollmentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// Paginated list of enrollments
#[derive(Debug, Clone, Deserialize)]
pub struct EnrollmentList {
    pub items: Vec<Enrollment>,
    pub meta: PaginationMeta,
}

/// Cancel enrollment result
#[derive(Debug, Clone, Deserialize)]
pub struct CancelEnrollmentResult {
    pub id: String,
    pub cancelled: bool,
}

/// Upload URL response
#[derive(Debug, Clone, Deserialize)]
pub struct UploadUrl {
    pub attachment_id: String,
    pub upload_url: String,
    pub expires_at: String,
}

/// Parameters for creating an upload URL
#[derive(Debug, Clone, Serialize)]
pub struct CreateUploadParams {
    pub filename: String,
    pub content_type: String,
    pub size: i64,
}

/// Attachment object
#[derive(Debug, Clone, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub status: String,
    pub created_at: String,
}

/// Cancel email result
#[derive(Debug, Clone, Deserialize)]
pub struct CancelEmailResult {
    pub id: String,
    pub cancelled: bool,
}

/// Add contact to list result
#[derive(Debug, Clone, Deserialize)]
pub struct AddContactResult {
    pub added: bool,
}

/// Remove contact from list result
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveContactResult {
    pub removed: bool,
}

/// Unsubscribe/resubscribe result
#[derive(Debug, Clone, Deserialize)]
pub struct SubscriptionResult {
    pub id: String,
    pub status: ContactStatus,
}
