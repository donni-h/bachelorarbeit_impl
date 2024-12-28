use std::future::Future;
use thiserror::Error;
use crate::domain::models::order_details::{SessionStatus, UserName};

pub trait CheckoutProducer: Clone + Send + Sync + 'static {
    fn notify_order_result(&self,
                           username: UserName,
                           status: SessionStatus,
    ) -> impl Future<Output=Result<(), NotifyError>> + Send;
}

#[derive(Error, Debug)]
pub enum NotifyError {
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
}