use std::future::Future;
use sea_orm::Order;
use crate::domain::models::metadata::{SessionId, UserName};

pub trait OrderRepository: Clone + Send + Sync + 'static {
    fn find_order_by_session_id(
        &self,
        req: &SessionId,
    ) -> impl Future<Output = Result<Order, FindOrderError>> + Send;

    fn find_orders_by_username(
        &self,
        req: &UserName,
    ) -> impl Future<Output = Result<Vec<Order>, FindOrderError>> + Send;

    fn create_order(
        &self,
        req: &Order,
    ) -> impl Future<Output = Result<Order, CreateOrderError>> + Send;

    fn delete_order(
        &self,
        req: uuid::Uuid,
    ) -> impl Future<Output = Result<(), DeleteOrderError>> + Send;

    fn delete_all_orders(
        &self,
    ) -> impl Future<Output = Result<(), DeleteOrderError>> + Send;
}