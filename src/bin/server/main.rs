use bachelorarbeit::domain::services::order_service::DefaultOrderService;
use bachelorarbeit::domain::services::payment_service::StripeService;
use bachelorarbeit::inbound::http::authorization::keycloak::fetch_jwk_set;
use bachelorarbeit::inbound::http::{HttpServer, HttpServerConfig};
use bachelorarbeit::outbound::postgres::Postgres;
use bachelorarbeit::outbound::rabbitmq::RabbitMQ;
use dotenv::dotenv;
use jsonwebtoken::{Algorithm, Validation};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let keycloak_url = std::env::var("KEYCLOAK_URL")
        .expect("KEYCLOAK_URL not set");
    let keycloak_issuer = std::env::var("KEYCLOAK_ISSUER")
        .expect("KEYCLOAK_ISSUER not set");
    let keys = fetch_jwk_set(&keycloak_url)
        .await
        .expect("Failed to fetch JWK Set");
    let secret_key = std::env::var("STRIPE_SK")
        .expect("missing stripe secret key");
    let postgres_url = std::env::var("DATABASE_URL")
        .expect("missing DATABASE_URL");
    let domain =  std::env::var("STRIPE_REDIRECT_URL")
        .expect("missing STRIPE_REDIRECT_URL not set");
    let rabbit_host = std::env::var("RABBIT_HOST").expect("missing RABBIT_HOST");


    let payment_service = Arc::new(
        StripeService::new(secret_key.clone(),
                           domain.to_string(),
        )
    );
    let postgres = Postgres::new(&postgres_url)
        .await
        .unwrap();
    let rabbit_mq = RabbitMQ::new(
        &rabbit_host,
        5672,
        "Checkout_ToBasket",
        "BasketExchange",
    )
        .await;
    let order_service = DefaultOrderService::new(
        postgres,
        rabbit_mq,
        payment_service.clone(),
    );
    let config = HttpServerConfig { port: "8080" };
    let mut validator = Validation::new(Algorithm::RS256);
    validator.set_issuer(&[keycloak_issuer]);
    validator.set_audience(&["account"]);

    HttpServer::new(
        order_service,
        payment_service,
        keys,
        validator,
        &config,
    )
        .await
        .expect("server crashed");
}
