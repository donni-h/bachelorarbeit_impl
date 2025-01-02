use std::sync::Arc;
use anyhow::{anyhow, Error};
use chrono::Utc;
use stripe::Object;
use uuid::Uuid;
use crate::domain::models::order_details::{OrderDetails, SessionId, SessionStatus, UserName};
use crate::domain::models::order::{CreateOrderError, CreateOrderRequest, DeleteOrderError, FindOrderError, Order, UpdateOrderError, UpdateOrderStatusRequest};
use crate::domain::models::order_item::OrderItem;
use crate::domain::ports::checkout_producer::CheckoutProducer;
use crate::domain::ports::order_repository::OrderRepository;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;

#[derive(Debug, Clone)]
pub struct DefaultOrderService<R, C, P>
where
    R: OrderRepository,
    C: CheckoutProducer,
    P: PaymentService,
{
    repository: R,
    checkout_producer: C,
    payment_service: Arc<P>
}

impl<R, C, P> DefaultOrderService<R, C, P>
where
    R: OrderRepository,
    C: CheckoutProducer,
    P: PaymentService,
{

    pub fn new(repository: R, checkout_producer: C, payment_service: Arc<P>) -> Self {
        Self{
            repository,
            checkout_producer,
            payment_service
        }
    }
}


impl<R, C, P> OrderService for DefaultOrderService<R, C, P>
where
     R: OrderRepository,
     C: CheckoutProducer,
     P: PaymentService,
 {
     async fn create_order(&self, req: &CreateOrderRequest) -> Result<String, CreateOrderError> {
         let status = Some(SessionStatus::Open);
         let created_at = Utc::now();

         let order_items = req.items()
             .iter()
             .map(|item| OrderItem::new(
                                        item.id().clone(),
                                        item.product_name().clone(), 
                                        item.item_id().clone(),
                                        item.price().clone())
             )
             .collect();

         let checkout_session = self.payment_service
             .create_checkout_session(&order_items)
             .await
             .map_err(|e| {
                 CreateOrderError::Unknown(anyhow!(e))
             })?;

         let session_id = SessionId::new(checkout_session.id().as_str());

         let details = OrderDetails::new(
             req.id().clone(),
             req.username().clone(),
             status,
             session_id,
             created_at,
         );

         let order = Order::new(details, order_items)?;

         let checkout_url = checkout_session
             .url
             .ok_or(CreateOrderError::Unknown(anyhow!("Couldn't get a checkout url")))?;

         let _ = self.repository.create_order(&order).await?;



         Ok(checkout_url)
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

     async fn notify_checkout_status(&self, req: &SessionId) -> Result<(), Error> {
         let order = self.repository.find_order_by_session_id(req).await?;

         let maybe_status = self.payment_service.retrieve_checkout_status(req).await?;

         if let Some(status) = maybe_status {
             self.checkout_producer.notify_order_result(order.details().username(), &status).await?;
             Ok(())
         } else { Err(anyhow!("Order doesn't have a checkout status")) }
     }


     async fn delete_order(&self, req: Uuid) -> Result<Uuid, DeleteOrderError> {
         self.repository.delete_order(req).await
     }

     async fn delete_all_orders(&self) -> Result<(), DeleteOrderError> {
         self.repository.delete_all_orders().await
         
     }

     async fn update_order_status(
         &self,
         req: UpdateOrderStatusRequest,
     ) -> Result<Order, UpdateOrderError> {
         self.repository.update_order_status(req.id(), req.status().as_ref()).await
     }
 }