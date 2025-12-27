use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pagination information returned with list endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: i32,
    pub limit: i32,
    pub total: i32,
    pub total_pages: i32,
    #[serde(default)]
    pub has_next: bool,
    #[serde(default)]
    pub has_prev: bool,
}

/// Email delivery status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum EmailStatus {
    #[default]
    Pending,
    Queued,
    Sent,
    Delivered,
    Bounced,
    Complained,
    Failed,
}

/// Result from sending an email
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEmailResult {
    pub message_id: String,
}

/// Email object (from list/get endpoints)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    /// Email ID (mapped from _id in API response)
    #[serde(alias = "_id", default)]
    pub id: String,
    #[serde(default)]
    pub message_id: Option<String>,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: Vec<String>,
    #[serde(default)]
    pub cc: Vec<String>,
    #[serde(default)]
    pub bcc: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub status: EmailStatus,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub email_type: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub sent_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub delivered_at: Option<String>,
}

/// Parameters for sending an email
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
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

/// Paginated list of emails (API returns {emails: [], pagination: {}})
#[derive(Debug, Clone, Deserialize)]
pub struct EmailList {
    pub emails: Vec<Email>,
    pub pagination: Pagination,
}

/// Email statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmailStats {
    pub total: i64,
    pub sent: i64,
    pub failed: i64,
    #[serde(default)]
    pub transactional: i64,
    #[serde(default)]
    pub marketing: i64,
    pub success_rate: f64,
}

/// Wrapper for email stats response from API
#[derive(Debug, Clone, Deserialize)]
pub struct EmailStatsResponse {
    pub stats: EmailStats,
}

/// Contact subscription status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ContactStatus {
    #[default]
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
#[serde(rename_all = "camelCase")]
pub struct Contact {
    #[serde(alias = "_id")]
    pub id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub status: ContactStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub source: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub subscribed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub unsubscribed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub consent_type: Option<ConsentType>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub consent_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub consent_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub consent_ip_address: Option<String>,
}

/// Parameters for creating a contact
#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateContactParams {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct UpdateContactParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
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
}

/// Paginated list of contacts (API returns {contacts: [], pagination: {}})
#[derive(Debug, Clone, Deserialize)]
pub struct ContactsResponse {
    pub contacts: Vec<Contact>,
    pub pagination: Pagination,
}

/// Reason for suppressing a contact
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SuppressReason {
    Manual,
    Unsubscribed,
    Bounced,
    Complained,
    SpamTrap,
}

/// Contact list object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
    #[serde(alias = "_id")]
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[serde(default)]
    pub total_contacts: i32,
    #[serde(default)]
    pub active_contacts: i32,
    #[serde(default)]
    pub suppressed_contacts: i32,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
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
}

/// Contact list statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStats {
    pub total_contacts: i64,
    pub active_contacts: i64,
    #[serde(default)]
    pub suppressed_contacts: i64,
}

/// Verification result status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VerificationStatus {
    Clean,
    Dirty,
    Valid,
    Invalid,
    Risky,
    Unknown,
}

/// Single email verification result
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult {
    pub email: String,
    pub status: VerificationStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    // Legacy fields (for compatibility)
    #[serde(default)]
    pub is_valid: bool,
    #[serde(default)]
    pub is_disposable: bool,
    #[serde(default)]
    pub is_role_based: bool,
    #[serde(default)]
    pub is_free_provider: bool,
    #[serde(default)]
    pub mx_found: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smtp_check: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

/// Categorized batch results (clean, dirty, unknown email lists)
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchResults {
    #[serde(default)]
    pub clean: Vec<String>,
    #[serde(default)]
    pub dirty: Vec<String>,
    #[serde(default)]
    pub unknown: Vec<String>,
}

/// Analytics for batch verification
#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BatchAnalytics {
    #[serde(default)]
    pub clean_count: i32,
    #[serde(default)]
    pub dirty_count: i32,
    #[serde(default)]
    pub unknown_count: i32,
    #[serde(default)]
    pub clean_percentage: f64,
}

/// Batch verification result
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchVerificationResult {
    /// For async batches, this is the verification ID to poll
    #[serde(default, alias = "verificationId")]
    pub verification_id: String,
    pub status: String,
    #[serde(default)]
    pub total: i32,
    #[serde(default)]
    pub total_emails: i32,
    #[serde(default)]
    pub processed: i32,
    #[serde(default)]
    pub credits_deducted: i32,
    /// Categorized results (for synchronous/completed batches)
    #[serde(default)]
    pub results: Option<BatchResults>,
    /// Analytics summary
    #[serde(default)]
    pub analytics: Option<BatchAnalytics>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// Verification statistics
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationStats {
    pub total_verified: i64,
    pub total_valid: i64,
    pub total_invalid: i64,
    pub total_unknown: i64,
    pub total_verifications: i64,
    pub valid_percentage: f64,
}

/// Verification list item (returned by list endpoint)
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationListItem {
    pub id: String,
    #[serde(rename = "type")]
    pub verification_type: String,
    pub status: String,
    #[serde(default)]
    pub total_emails: i32,
    #[serde(default)]
    pub progress: i32,
    #[serde(default)]
    pub analytics: Option<BatchAnalytics>,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
}

/// Response from verification list endpoint
#[derive(Debug, Clone, Deserialize)]
pub struct VerificationListResponse {
    pub items: Vec<VerificationListItem>,
}

/// Upload URL response
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadUrl {
    pub attachment_id: String,
    pub upload_url: String,
    pub expires_at: String,
}

/// Parameters for creating an upload URL
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUploadParams {
    pub filename: String,
    pub content_type: String,
    pub size: i64,
}

/// Attachment object
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
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

/// Suppress params for contact suppression
#[derive(Debug, Clone, Serialize)]
pub struct SuppressParams {
    pub reason: SuppressReason,
}

/// Paginated list of contact lists
#[derive(Debug, Clone, Deserialize)]
pub struct ListsResponse {
    pub lists: Vec<List>,
    pub pagination: Pagination,
}
