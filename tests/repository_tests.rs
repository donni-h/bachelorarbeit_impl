use std::str::FromStr;
use anyhow::Context;
use chrono::Utc;
use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;
use testcontainers_modules::postgres::Postgres as PostgreContainer;
use testcontainers_modules::testcontainers::ContainerAsync;
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use uuid::Uuid;
use bachelorarbeit::domain::models::order::{Order};
use bachelorarbeit::domain::models::order_details::{OrderDetails, SessionId, SessionStatus, UserName};
use bachelorarbeit::domain::models::order_item::{OrderItem, Price, ProductName};
use bachelorarbeit::domain::ports::order_repository::OrderRepository;
use bachelorarbeit::outbound::postgres::Postgres;


async fn setup_repository() -> (impl OrderRepository, ContainerAsync<PostgreContainer>) {
    let container = PostgreContainer::default()
        .start()
        .await
        .unwrap();
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .unwrap();

    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",
    );

    let repository = Postgres::new(connection_string).await.unwrap();
    sqlx::migrate!("./migrations").run(repository.pool()).await.unwrap();

    (repository, container)
}

    fn get_mock_create_order() -> Order {
    let id = Uuid::default();

    let username = UserName::new("Hannes");
    let session_status = Some(SessionStatus::Open);
    let session_id = SessionId::new("abc123");
    let created_at = Utc::now();
    let details = OrderDetails::new(
        id,
        username,
        session_status,
        session_id,
        created_at,
    );

    let product_name = ProductName::new("Produkt");
    let price = Price::new(1.0).unwrap();
    let item = OrderItem::new(
        id,
        product_name,
        id,
        price
    );

    Order::new(
        details,
        vec![item],
    ).unwrap()
}

#[tokio::test]
async fn test_repository_connection() {
    let container = PostgreContainer::default()
        .start()
        .await
        .unwrap();
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .unwrap();

    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",
    );

    let repository_result = Postgres::new(connection_string).await;

    assert!(repository_result.is_ok());
}

#[tokio::test]
async fn test_repository_migrations() {
    let container = PostgreContainer::default()
        .start()
        .await
        .unwrap();
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .unwrap();

    let connection_string = &format!(
        "postgres://postgres:postgres@127.0.0.1:{host_port}/postgres",
    );

    let pool  = PgPool::connect_with(
        PgConnectOptions::from_str(connection_string)
            .unwrap()
    )
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let table_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public'")
        .fetch_one(&pool)
        .await
        .expect("Failed to get table count");

    assert!(table_count > 0, "No tables were created by migrations");
}

#[tokio::test]
async fn test_create_order() {

    let (repository, _container) = setup_repository().await;
    let order = get_mock_create_order();
    let id = repository.create_order(&order).await.unwrap();
    assert_eq!(id, Uuid::default());
}