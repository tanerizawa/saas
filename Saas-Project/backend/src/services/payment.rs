use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::domain::value_objects::{UserId, CompanyId};

// Payment-related data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub user_id: UserId,
    pub amount: f64,
    pub currency: String,
    pub payment_type: PaymentType,
    pub description: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentType {
    InitialPayment,
    SubscriptionPayment,
    LicenseFee,
    AdditionalService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub payment_id: Uuid,
    pub status: PaymentStatus,
    pub gateway_transaction_id: Option<String>,
    pub payment_url: Option<String>,
    pub redirect_url: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Success,
    Failed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentWebhook {
    pub gateway: PaymentGateway,
    pub transaction_id: String,
    pub status: PaymentStatus,
    pub amount: f64,
    pub currency: String,
    pub metadata: HashMap<String, String>,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentGateway {
    Midtrans,
    Xendit,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub billing_cycle: BillingCycle,
    pub features: Vec<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingCycle {
    Monthly,
    Quarterly,
    Annually,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub plan_id: Uuid,
    pub status: SubscriptionStatus,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    Active,
    PastDue,
    Cancelled,
    Trialing,
    Incomplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub user_id: UserId,
    pub company_id: CompanyId,
    pub invoice_number: String,
    pub amount: f64,
    pub currency: String,
    pub status: InvoiceStatus,
    pub due_date: DateTime<Utc>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Void,
    Uncollectible,
}

// Service implementation
pub struct PaymentService {
    // TODO: Add actual payment gateway clients
    // midtrans_client: Arc<MidtransClient>,
    // xendit_client: Arc<XenditClient>,
}

impl PaymentService {
    pub fn new() -> Self {
        Self {
            // TODO: Initialize payment gateway clients
        }
    }

    /// Create a new payment request
    pub async fn create_payment(
        &self,
        request: PaymentRequest,
    ) -> Result<PaymentResponse, PaymentError> {
        // Validate payment request
        self.validate_payment_request(&request)?;

        // Generate payment ID
        let payment_id = Uuid::new_v4();

        // TODO: Integrate with actual payment gateway
        // For now, return a mock response for development
        let response = PaymentResponse {
            payment_id,
            status: PaymentStatus::Pending,
            gateway_transaction_id: Some(format!("mock_txn_{}", payment_id)),
            payment_url: Some(format!("https://payment-gateway.mock/pay/{}", payment_id)),
            redirect_url: Some("https://your-app.com/payment/callback".to_string()),
            expires_at: Some(Utc::now() + chrono::Duration::hours(24)),
            created_at: Utc::now(),
        };

        // TODO: Store payment in database
        // self.repository.create_payment(&request, &response).await?;

        Ok(response)
    }

    /// Handle payment webhook from gateway
    pub async fn handle_webhook(
        &self,
        webhook: PaymentWebhook,
    ) -> Result<(), PaymentError> {
        // Verify webhook signature (important for security)
        self.verify_webhook_signature(&webhook)?;

        // TODO: Update payment status in database
        // self.repository.update_payment_status(&webhook.transaction_id, webhook.status).await?;

        // Handle status-specific logic
        match webhook.status {
            PaymentStatus::Success => {
                self.handle_successful_payment(&webhook).await?;
            }
            PaymentStatus::Failed => {
                self.handle_failed_payment(&webhook).await?;
            }
            PaymentStatus::Cancelled => {
                self.handle_cancelled_payment(&webhook).await?;
            }
            _ => {
                // Log other status updates
                log::info!("Payment status updated: {:?}", webhook.status);
            }
        }

        Ok(())
    }

    /// Create a new subscription
    pub async fn create_subscription(
        &self,
        user_id: UserId,
        company_id: CompanyId,
        plan_id: Uuid,
    ) -> Result<Subscription, PaymentError> {
        // TODO: Validate plan exists and is active
        
        let subscription = Subscription {
            id: Uuid::new_v4(),
            user_id,
            company_id,
            plan_id,
            status: SubscriptionStatus::Active,
            current_period_start: Utc::now(),
            current_period_end: Utc::now() + chrono::Duration::days(30), // Default to monthly
            cancelled_at: None,
            trial_end: Some(Utc::now() + chrono::Duration::days(7)), // 7-day trial
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // TODO: Store subscription in database
        // self.repository.create_subscription(&subscription).await?;

        Ok(subscription)
    }

    /// Generate invoice for subscription
    pub async fn generate_invoice(
        &self,
        subscription_id: Uuid,
    ) -> Result<Invoice, PaymentError> {
        // TODO: Get subscription from database
        // let subscription = self.repository.get_subscription(&subscription_id).await?;

        let invoice = Invoice {
            id: Uuid::new_v4(),
            subscription_id,
            user_id: UserId::new(), // TODO: Get from subscription
            company_id: CompanyId::new(), // TODO: Get from subscription
            invoice_number: format!("INV-{}", Utc::now().format("%Y%m%d-%H%M%S")),
            amount: 299000.0, // TODO: Get from plan
            currency: "IDR".to_string(),
            status: InvoiceStatus::Open,
            due_date: Utc::now() + chrono::Duration::days(30),
            paid_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // TODO: Store invoice in database
        // self.repository.create_invoice(&invoice).await?;

        Ok(invoice)
    }

    /// Get payment status
    pub async fn get_payment_status(
        &self,
        payment_id: &Uuid,
    ) -> Result<PaymentStatus, PaymentError> {
        // TODO: Get payment from database
        // let payment = self.repository.get_payment(payment_id).await?;
        
        // For now, return mock status
        Ok(PaymentStatus::Pending)
    }

    /// Get user subscriptions
    pub async fn get_user_subscriptions(
        &self,
        user_id: &UserId,
    ) -> Result<Vec<Subscription>, PaymentError> {
        // TODO: Get subscriptions from database
        // self.repository.get_user_subscriptions(user_id).await
        
        Ok(vec![])
    }

    /// Cancel subscription
    pub async fn cancel_subscription(
        &self,
        subscription_id: Uuid,
        reason: Option<String>,
    ) -> Result<(), PaymentError> {
        // TODO: Update subscription status in database
        // self.repository.cancel_subscription(&subscription_id, reason).await?;

        // TODO: Send cancellation email
        // self.email_service.send_cancellation_email().await?;

        Ok(())
    }

    // Private helper methods

    fn validate_payment_request(&self, request: &PaymentRequest) -> Result<(), PaymentError> {
        if request.amount <= 0.0 {
            return Err(PaymentError::InvalidAmount);
        }

        if request.currency.is_empty() {
            return Err(PaymentError::InvalidCurrency);
        }

        // Add more validations as needed
        Ok(())
    }

    fn verify_webhook_signature(&self, webhook: &PaymentWebhook) -> Result<(), PaymentError> {
        // TODO: Implement actual signature verification
        // This is critical for security to prevent fake webhooks
        match webhook.gateway {
            PaymentGateway::Midtrans => {
                // Verify Midtrans signature
                Ok(())
            }
            PaymentGateway::Xendit => {
                // Verify Xendit signature
                Ok(())
            }
            PaymentGateway::Manual => {
                // Manual payments don't have webhooks
                Ok(())
            }
        }
    }

    async fn handle_successful_payment(&self, webhook: &PaymentWebhook) -> Result<(), PaymentError> {
        // TODO: Activate subscription or complete onboarding
        // TODO: Send success notification email
        // TODO: Update license status if applicable
        
        log::info!("Payment successful: {}", webhook.transaction_id);
        Ok(())
    }

    async fn handle_failed_payment(&self, webhook: &PaymentWebhook) -> Result<(), PaymentError> {
        // TODO: Send failure notification
        // TODO: Retry logic for subscription payments
        // TODO: Grace period handling
        
        log::warn!("Payment failed: {}", webhook.transaction_id);
        Ok(())
    }

    async fn handle_cancelled_payment(&self, webhook: &PaymentWebhook) -> Result<(), PaymentError> {
        // TODO: Handle payment cancellation
        // TODO: Update onboarding status if applicable
        
        log::info!("Payment cancelled: {}", webhook.transaction_id);
        Ok(())
    }
}

// Error handling
#[derive(Debug)]
pub enum PaymentError {
    InvalidAmount,
    InvalidCurrency,
    PaymentGatewayError(String),
    WebhookVerificationFailed,
    SubscriptionNotFound,
    InvoiceNotFound,
    DatabaseError(String),
    NetworkError(String),
}

impl std::fmt::Display for PaymentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentError::InvalidAmount => write!(f, "Invalid payment amount"),
            PaymentError::InvalidCurrency => write!(f, "Invalid currency"),
            PaymentError::PaymentGatewayError(msg) => write!(f, "Payment gateway error: {}", msg),
            PaymentError::WebhookVerificationFailed => write!(f, "Webhook verification failed"),
            PaymentError::SubscriptionNotFound => write!(f, "Subscription not found"),
            PaymentError::InvoiceNotFound => write!(f, "Invoice not found"),
            PaymentError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            PaymentError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for PaymentError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_validation() {
        let service = PaymentService::new();
        
        let invalid_request = PaymentRequest {
            user_id: UserId::new(),
            amount: -100.0,
            currency: "".to_string(),
            payment_type: PaymentType::InitialPayment,
            description: "Test payment".to_string(),
            metadata: HashMap::new(),
        };

        assert!(service.validate_payment_request(&invalid_request).is_err());
    }

    #[test] 
    fn test_payment_types() {
        // Test serialization/deserialization
        let payment_type = PaymentType::SubscriptionPayment;
        let serialized = serde_json::to_string(&payment_type).unwrap();
        assert!(serialized.contains("SubscriptionPayment"));
    }
}
