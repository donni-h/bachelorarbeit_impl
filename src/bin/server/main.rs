use actix_web::error::DispatchError::Service;
use chrono::Utc;
use uuid::Uuid;
use bachelorarbeit::domain::models::order::{CreateOrderRequest, Order};
use bachelorarbeit::domain::models::order_details::{OrderDetails, SessionId, UserName};
use bachelorarbeit::domain::models::order_details::SessionStatus;
use bachelorarbeit::domain::models::order_item::{CreateOrderItemRequest, OrderItem, Price, ProductName};
use bachelorarbeit::domain::ports::order_repository::OrderRepository;
use bachelorarbeit::domain::ports::order_service::OrderService;
use bachelorarbeit::domain::services::order_service::DefaultOrderService;
use bachelorarbeit::outbound::postgres::Postgres;
use bachelorarbeit::outbound::rabbitmq::RabbitMQ;

#[tokio::main]
async fn main() {
    let postgres = Postgres::new("postgres://admin:admin@localhost:5432/bachelorarbeit").await.unwrap();
    let price = Price::new(4.0).unwrap();
    let product_name = ProductName::new("mein Produkt");
    let username = UserName::new("Hannes");
    let item_req = CreateOrderItemRequest::new(product_name.clone(), Uuid::new_v4(), price.clone());
    let item_req2 = CreateOrderItemRequest::new(product_name.clone(), Uuid::new_v4(), price.clone());
    let item_req3 = CreateOrderItemRequest::new(product_name, Uuid::new_v4(), price);
    let service = DefaultOrderService::new(postgres, RabbitMQ::new());

    let req = CreateOrderRequest::new(username.clone(),vec![item_req,item_req2,item_req3]);
    let id_result = service.create_order(&req).await;
    
    println!("{:#?}", id_result); 
    
    let orders_result = service.find_orders_by_username(&username).await;
    println!("{:#?}", orders_result);
    
    println!("{:#?}", orders_result.unwrap().len());
    let res = service.delete_order(id_result.unwrap().details().order_id().clone()).await.unwrap();
    println!("{:#?}", res);
} 
