use chrono::Utc;
use uuid::Uuid;
use bachelorarbeit::domain::models::order::Order;
use bachelorarbeit::domain::models::order_details::{OrderDetails, SessionId, UserName};
use bachelorarbeit::domain::models::order_details::SessionStatus;
use bachelorarbeit::domain::models::order_item::{OrderItem, Price, ProductName};
use bachelorarbeit::domain::ports::order_repository::OrderRepository;
use bachelorarbeit::outbound::postgres::Postgres;

#[tokio::main]
async fn main() {
    let postgres = Postgres::new("postgres://admin:admin@localhost:5432/bachelorarbeit").await.unwrap();
    let price = Price::new(4.0).unwrap();
    let product_name = ProductName::new("mein Produkt");
    let order_item = OrderItem::new(Uuid::new_v4(), product_name, Uuid::new_v4(), price);
    
    
    let order_id = Uuid::new_v4();
    let session_id = SessionId::new("abcd1234");
    let username = UserName::new("Hannes");
    let status = Some(SessionStatus::Open);
    let created_at = Utc::now();
    let details = OrderDetails::new(order_id, username, status, session_id, created_at);
    
    let order = Order::new(details, vec![order_item]);
    
    let id = postgres.create_order(&order).await.expect("Couldn't create item");
    
    println!("{:?}", id);
    
    let id = postgres.delete_order(id).await.expect("Couldn't delete order");
    
    println!("{:?}", id);
}
