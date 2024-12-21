use bachelorarbeit::domain::models::metadata::{SessionId, UserName};
use bachelorarbeit::domain::models::metadata::SessionStatus;
use bachelorarbeit::outbound::postgres::Postgres;

#[tokio::main]
async fn main() {
    let postgres = Postgres::new("postgres://admin:admin@localhost:5432/bachelorarbeit").await.unwrap();
    let id = SessionId::new("abcd1234");
    let result = postgres.find_metadata_by_session_id(&id).await.unwrap();

    println!("metadata: {:?}", result);
}
