use std::future::Future;
use crate::domain::models::order_details::{SessionId, UserName};
use crate::domain::models::order::{CreateOrderError, CreateOrderRequest, DeleteOrderError, FindOrderError, Order};

pub trait OrderService: Clone + Send + Sync + 'static {

  fn create_order(
        &self,
        req: &CreateOrderRequest,
    ) -> impl Future<Output = Result<Order, CreateOrderError>> + Send;

   fn find_order_by_session_id(
        &self,
        req: &SessionId,
    ) -> impl Future<Output = Result<Order, FindOrderError>> + Send;

    fn find_orders_by_username(
        &self,
        req: &UserName,
    ) -> impl Future<Output = Result<Vec<Order>, FindOrderError>> + Send;
    
    /*
    fn notify_checkout_status(
        &self,
        req: &SessionId,
    ) -> impl Future<Output = Result<(), anyhow::Error>> + Send;

    fn delete_order(
        &self,
        req: uuid::Uuid,
    ) -> impl Future<Output = Result<(), DeleteOrderError>> + Send;
    */
    
    fn delete_all_orders(
        &self,
    ) -> impl Future<Output = Result<(), DeleteOrderError>> + Send;
}

