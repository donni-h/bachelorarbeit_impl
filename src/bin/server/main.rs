use std::str::FromStr;
use std::sync::Arc;
use actix_web::error::DispatchError::Service;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use stripe::CheckoutSessionId;
use uuid::Uuid;
use bachelorarbeit::domain::models::order::{CreateOrderRequest, Order, UpdateOrderStatusRequest};
use bachelorarbeit::domain::models::order_details::{OrderDetails, SessionId, UserName};
use bachelorarbeit::domain::models::order_details::SessionStatus;
use bachelorarbeit::domain::models::order_item::{CreateOrderItemRequest, OrderItem, Price, ProductName};
use bachelorarbeit::domain::ports::checkout_producer::CheckoutProducer;
use bachelorarbeit::domain::ports::order_repository::OrderRepository;
use bachelorarbeit::domain::ports::order_service::OrderService;
use bachelorarbeit::domain::ports::payment_service::PaymentService;
use bachelorarbeit::domain::services::order_service::DefaultOrderService;
use bachelorarbeit::domain::services::payment_service::StripeService;
use bachelorarbeit::outbound::postgres::Postgres;
use bachelorarbeit::outbound::rabbitmq::{RabbitMQ};

#[tokio::main]
async fn main() {
    let secret_key = std::env::var("STRIPE_SK").expect("missing stripe secret key");
    let domain = "http://127.0.0.1:8080";
    let payment_service = Arc::new(StripeService::new(secret_key.clone(), domain.to_string()));
    let postgres = Postgres::new("postgres://admin:admin@localhost:5432/bachelorarbeit").await.unwrap();
    let price = Price::new(4.0).unwrap();
    let product_name = ProductName::new("mein Produkt");
    let username = UserName::new("Tobias");
    let rabbit_mq =
        RabbitMQ::new("127.0.0.1", 5672, "Checkout_ToBasket", "BasketExchange")
        .await;
    let item_req = CreateOrderItemRequest::new(product_name.clone(), Uuid::new_v4(), price.clone());
    let item_req2 = CreateOrderItemRequest::new(product_name.clone(), Uuid::new_v4(), price.clone());
    let item_req3 = CreateOrderItemRequest::new(product_name, Uuid::new_v4(), price);
    
    rabbit_mq.notify_order_result(username.clone(), SessionStatus::Complete).await.expect("TODO: panic message");


}
