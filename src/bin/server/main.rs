use bachelorarbeit::domain::models::order_details::{SessionId, UserName};
use bachelorarbeit::domain::models::order_details::SessionStatus;
use bachelorarbeit::outbound::postgres::Postgres;

#[tokio::main]
async fn main() {
    let postgres = Postgres::new("postgres://admin:admin@localhost:5432/bachelorarbeit").await.unwrap();
    let id = SessionId::new("session-1111");
    let result = postgres.find_order_details_by_session_id(&id).await.unwrap();

    println!("metadata: {:?}", result);
}
