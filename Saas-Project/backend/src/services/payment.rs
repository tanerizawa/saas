use crate::domain::value_objects::Money;
use crate::shared::errors::AppResult;

/// Service for processing payments. This is a very small stub used in tests and
/// example code. Real integrations with payment gateways would live here.
#[derive(Debug, Default)]
pub struct PaymentService;

impl PaymentService {
    #[allow(dead_code)]
    /// Create a new service instance
    pub fn new() -> Self {
        Self
    }

    /// Charge a payment of the given amount. The default implementation simply
    /// returns `Ok(())` and does not perform any external calls.
    #[allow(dead_code)]
    pub fn charge(&self, amount: Money) -> AppResult<()> {
        let _ = amount;
        Ok(())
    }
}
