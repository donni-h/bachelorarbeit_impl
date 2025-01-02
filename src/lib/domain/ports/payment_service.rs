use std::future::Future;
use stripe::CheckoutSession;
use thiserror::Error;
use crate::domain::models::order_details::{SessionId, SessionStatus};
use crate::domain::models::order_item::OrderItem;

pub trait PaymentService: Clone + Send + Sync + 'static {
    fn create_checkout_session(
        &self,
        order_items: &Vec<OrderItem>,
    ) -> impl Future<Output=Result<CheckoutSession, PaymentServiceError>> + Send;

    fn retrieve_checkout_status(
        &self,
        id: &SessionId,
    ) -> impl Future<Output=Result<Option<SessionStatus>, PaymentServiceError>> + Send;
    
    fn expire_session(
        &self,
        id: &SessionId,
    ) -> impl Future<Output=Result<(), PaymentServiceError>> + Send;
}

#[derive(Debug, Error)]
pub enum PaymentServiceError {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    #[error("invalid session id {0}")]
    InvalidSessionId(SessionId),
}

