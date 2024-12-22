use std::future::Future;
use uuid::Uuid;
use crate::domain::models::order_details::{SessionId, UserName};
use crate::domain::models::order::{CreateOrderError, DeleteOrderError, FindOrderError, Order};
use crate::domain::ports::checkout_producer::CheckoutProducer;
use crate::domain::ports::order_repository::OrderRepository;
use crate::domain::ports::order_service::OrderService;

#[derive(Debug, Clone)]
pub struct DefaultOrderService<R, C>
where
    R: OrderRepository,
    C: CheckoutProducer,
{
    repository: R,
    checkout_producer: C,
}

impl<R, C> DefaultOrderService<R, C>
where
    R: OrderRepository,
    C: CheckoutProducer,
{

    pub fn new(repository: R, checkout_producer: C) -> Self {
        Self{
            repository,
            checkout_producer,
        }
    }
}


// impl<R, C> OrderService for DefaultOrderService<R, C>
// where
//     R: OrderRepository,
//     C: CheckoutProducer,
// {
//     fn find_order_by_session_id(&self, req: &SessionId) -> impl Future<Output=Result<Order, FindOrderError>> + Send {
//         todo!()
//     }
//
//     fn find_orders_by_username(&self, req: &UserName) -> impl Future<Output=Result<Vec<Order>, FindOrderError>> + Send {
//         todo!()
//     }
//
//     fn create_order(&self, req: &Order) -> impl Future<Output=Result<Order, CreateOrderError>> + Send {
//         todo!()
//     }
//
//     fn notify_checkout_status(&self, req: &SessionId) -> Result<(), NotifyCheckoutError> {
//         todo!()
//     }
//
//     fn delete_order(&self, req: Uuid) -> impl Future<Output=Result<(), DeleteOrderError>> + Send {
//         todo!()
//     }
//
//     fn delete_all_orders(&self) -> impl Future<Output=Result<(), DeleteOrderError>> + Send {
//         todo!()
//     }
// }