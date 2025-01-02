use std::future::Future;
use uuid::Uuid;
use crate::domain::models::order_details::{SessionId, SessionStatus, UserName};
use crate::domain::models::order::{CreateOrderError, DeleteOrderError, FindOrderError, Order, UpdateOrderError};

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
     ) -> impl Future<Output = Result<Uuid, CreateOrderError>> + Send;
    
    fn delete_order(
         &self,
         req: uuid::Uuid,
    ) -> impl Future<Output = Result<uuid::Uuid, DeleteOrderError>> + Send;
    
    fn delete_all_orders(
         &self,
     ) -> impl Future<Output = Result<(), DeleteOrderError>> + Send;
    
    fn find_order_by_id(
        &self,
        req: Uuid,
    ) -> impl Future<Output = Result<Order, FindOrderError>> + Send;

    fn update_order_status(
        &self,
        id: &Uuid,
        status: Option<&SessionStatus>,
    ) -> impl Future<Output=Result<Order, UpdateOrderError>> + Send;
}