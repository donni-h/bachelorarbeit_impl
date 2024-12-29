use std::sync::Arc;
use actix_web::{web, HttpResponse};
use actix_web::web::{Data, ServiceConfig};
use anyhow::Context;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::domain::services::order_service::DefaultOrderService;
use crate::domain::services::payment_service::StripeService;
use crate::inbound::http::handlers::create_checkout::create_checkout;
use crate::outbound::postgres::Postgres;
use crate::outbound::rabbitmq::RabbitMQ;

mod handlers;
mod responses;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
pub struct AppState<OS: OrderService, PS: PaymentService> {
    order_service: Arc<OS>,
    payment_service: Arc<PS>,
}

pub struct HttpServer;

impl HttpServer {

    pub async fn new(
        order_service: impl OrderService,
        payment_service: Arc<impl PaymentService>,
        config: &HttpServerConfig<'_>,
    ) -> anyhow::Result<()> {

        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        let state = Data::new(AppState {
            order_service: Arc::new(order_service),
            payment_service,
        });

        actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .wrap(actix_web::middleware::Logger::default())
                .app_data(state.clone())
                .configure(api_routes)
        })
            .bind(format!("0.0.0.0:{}", config.port))
            .with_context(|| format!("Failed to bind to {}", config.port))?
            .run()
            .await
            .context("failed to run the server")
    }
}

fn api_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/payment")
            .route("/test", web::get().to(|| async { HttpResponse::Ok() }))
            .route("/create-checkout-session", web::post().to(create_checkout::<DefaultOrderService<Postgres, RabbitMQ, StripeService>, StripeService>))
    );
}
