use std::future::Future;
use std::str::FromStr;
use stripe::{CheckoutSession, CheckoutSessionId, Client, StripeError};
use crate::domain::models::order::CreateOrderRequest;
use crate::domain::models::order_details::{SessionId, SessionStatus};
use crate::domain::ports::checkout_producer::CheckoutProducer;
use crate::domain::ports::order_repository::OrderRepository;
use crate::domain::ports::payment_service::PaymentService;

#[derive(Clone)]
pub struct StripeService {
    client: Client,
    domain: String,
}

impl StripeService {
    fn new(secret: String, domain: String) -> Self {
        let client = Client::new(secret);
        
        Self { client, domain }
    }
}

impl PaymentService for StripeService {
    async fn create_checkout_session(&self, order: &CreateOrderRequest) -> Result<CheckoutSession, StripeError> {
        todo!()
    }

    async fn retrieve_checkout_status(&self, id: &SessionId) -> Result<SessionStatus, StripeError> {
        todo!()
    }

    async fn expire_session(&self, id: &SessionId) -> Result<(), StripeError> {
        let session_id = CheckoutSessionId::from_str(&id.to_string())?;
        let _ = CheckoutSession::expire(&self.client, &session_id).await?;
        
        Ok(())
    }
}