use std::str::FromStr;
use std::sync::Arc;
use actix_web::error::DispatchError::Service;
use chrono::Utc;
use dotenv::dotenv;
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
use bachelorarbeit::inbound::http::{HttpServer, HttpServerConfig};
use bachelorarbeit::outbound::postgres::Postgres;
use bachelorarbeit::outbound::rabbitmq::{RabbitMQ};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let secret_key = std::env::var("STRIPE_SK").expect("missing stripe secret key");
    let postgres_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let domain = "http://127.0.0.1:8080";
    let payment_service = Arc::new(StripeService::new(secret_key.clone(), domain.to_string()));
    let postgres = Postgres::new(&postgres_url).await.unwrap();
    let rabbit_mq =
        RabbitMQ::new("127.0.0.1", 5672, "Checkout_ToBasket", "BasketExchange")
        .await;

    let order_service = DefaultOrderService::new(postgres, rabbit_mq, payment_service.clone());

    let config = HttpServerConfig {port: "8080"};
    
    HttpServer::new(order_service, payment_service, &config).await.expect("server crashed");
    
}
