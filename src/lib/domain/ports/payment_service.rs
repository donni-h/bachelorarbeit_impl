use std::future::Future;
use stripe::{CheckoutSession, StripeError};
use crate::domain::models::order::{CreateOrderError, CreateOrderRequest, Order};
use crate::domain::models::order_details::{SessionId, SessionStatus};

pub trait PaymentService: Clone + Send + Sync + 'static {
    fn create_checkout_session(
        &self,
        order: &CreateOrderRequest,
    ) -> impl Future<Output=Result<CheckoutSession, StripeError>> + Send;

    fn retrieve_checkout_status(
        &self,
        id: &SessionId,
    ) -> impl Future<Output=Result<SessionStatus, StripeError>> + Send;
    
    fn expire_session(
        &self,
        id: &SessionId,
    ) -> impl Future<Output=Result<(), StripeError>> + Send;
}