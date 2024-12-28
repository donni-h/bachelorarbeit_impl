use std::future::Future;
use chrono::Utc;
use uuid::Uuid;
use crate::domain::models::order_details::{OrderDetails, SessionId, SessionStatus, UserName};
use crate::domain::models::order::{CreateOrderError, CreateOrderRequest, DeleteOrderError, FindOrderError, Order};
use crate::domain::models::order_item::OrderItem;
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


impl<R, C> OrderService for DefaultOrderService<R, C>
where
     R: OrderRepository,
     C: CheckoutProducer,
 {
     async fn create_order(&self, req: &CreateOrderRequest) -> Result<Order, CreateOrderError> {
         let status = Some(SessionStatus::Open);
         let session_id = SessionId::new("my session");
         let created_at = Utc::now();
         
         let details = OrderDetails::new(
             req.id().clone(),
             req.username().clone(),
             status,
             session_id,
             created_at,
         );
         
         let order_items = req.items()
             .iter()
             .map(|item| OrderItem::new(
                                        item.id().clone(),
                                        item.product_name().clone(), 
                                        item.item_id().clone(),
                                        item.price().clone())
             )
             .collect();
             
         let order = Order::new(details, order_items)?;
         
         let _ = self.repository.create_order(&order).await?;

         Ok(order)
     }
     async fn find_order_by_session_id(&self, req: &SessionId) -> Result<Order, FindOrderError> {
         self.repository.find_order_by_session_id(req).await
     }
//
     async fn find_orders_by_username(&self, req: &UserName) -> Result<Vec<Order>, FindOrderError> {
         self.repository.find_orders_by_username(req).await
     }

     async fn find_order_by_id(&self, req: Uuid) -> Result<Order, FindOrderError> {
         self.repository.find_order_by_id(req).await
     }

   
//     fn notify_checkout_status(&self, req: &SessionId) -> Result<(), NotifyCheckoutError> {
//         todo!()
//     }
//
     async fn delete_order(&self, req: Uuid) -> Result<Uuid, DeleteOrderError> {
         self.repository.delete_order(req).await
     }

     async fn delete_all_orders(&self) -> Result<(), DeleteOrderError> {
         self.repository.delete_all_orders().await
         
     }
}