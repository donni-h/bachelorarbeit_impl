use std::collections::HashMap;
use std::sync::Arc;
use actix_web::web;
use actix_web::web::{Data, ServiceConfig};
use anyhow::Context;
use jsonwebtoken::{DecodingKey, Validation};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::domain::ports::order_service::OrderService;
use crate::domain::ports::payment_service::PaymentService;
use crate::domain::services::order_service::DefaultOrderService;
use crate::domain::services::payment_service::StripeService;
use crate::inbound::http::handlers::cancel::cancel;
use crate::inbound::http::handlers::create_checkout::{create_checkout, CreateOrderHttpRequestBody};
use crate::inbound::http::handlers::delete_all_orders::delete_all_orders;
use crate::inbound::http::handlers::delete_by_id::delete_order_by_id;
use crate::inbound::http::handlers::get_by_id::get_order_by_id;
use crate::inbound::http::handlers::success::success;
use crate::outbound::postgres::Postgres;
use crate::outbound::rabbitmq::RabbitMQ;
use crate::inbound::http::handlers::create_checkout::__path_create_checkout;
use crate::inbound::http::handlers::cancel::__path_cancel;
use crate::inbound::http::handlers::delete_all_orders::__path_delete_all_orders;
use crate::inbound::http::handlers::delete_by_id::__path_delete_order_by_id;
use crate::inbound::http::handlers::get_all_orders_for_user::get_all_orders_for_user;
use crate::inbound::http::handlers::get_by_id::__path_get_order_by_id;
use crate::inbound::http::handlers::success::__path_success;
use crate::inbound::http::handlers::get_all_orders_for_user::__path_get_all_orders_for_user;
use crate::inbound::http::responses::OrderResponseData;
mod handlers;
mod responses;
mod extractors;
pub mod authorization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Clone, Debug)]
pub struct AppState<OS: OrderService, PS: PaymentService> {
    order_service: Arc<OS>,
    payment_service: Arc<PS>,
}

pub struct AuthState {
    auth_keys: Arc<HashMap<String,DecodingKey>>,
    validator: Arc<Validation>,
}
pub struct HttpServer;

impl HttpServer {

    pub async fn new(
        order_service: impl OrderService,
        payment_service: Arc<impl PaymentService>,
        auth_key: HashMap<String,DecodingKey>,
        validator: Validation,
        config: &HttpServerConfig<'_>,
    ) -> anyhow::Result<()> {

        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        let app_state = Data::new(AppState {
            order_service: Arc::new(order_service),
            payment_service,
        });

        let openapi = ApiDoc::openapi();

        let auth_state = Data::new(AuthState {
            auth_keys: Arc::new(auth_key),
            validator: Arc::new(validator),
        });
        actix_web::HttpServer::new(move || {
            actix_web::App::new()
                .wrap(actix_web::middleware::Logger::default())
                .app_data(app_state.clone())
                .app_data(auth_state.clone())
                .configure(api_routes)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}")
                        .url("/api-docs/openapi.json", openapi.clone())
                )
        })
            .bind(format!("0.0.0.0:{}", config.port))
            .with_context(|| format!("Failed to bind to {}", config.port))?
            .run()
            .await
            .context("failed to run the server")
    }
}

fn api_routes(cfg: &mut ServiceConfig) {
    type OrderService = DefaultOrderService<Postgres, RabbitMQ, StripeService>;
    type PaymentService = StripeService;
    cfg.service(
        web::scope("/api/payment")
            .route("/create-checkout-session", web::post().to(create_checkout::<OrderService, PaymentService>))
            .route("/success", web::get().to(success::<OrderService, PaymentService>))
            .route("/cancel", web::get().to(cancel::<OrderService, PaymentService>))
            .route("/orderbyid", web::get().to(get_order_by_id::<OrderService, PaymentService>))
            .route("/allordersforuser", web::get().to(get_all_orders_for_user::<OrderService, PaymentService>))
            .route("/order", web::delete().to(delete_order_by_id::<OrderService, PaymentService>))
            .route("/orders", web::delete().to(delete_all_orders::<OrderService, PaymentService>))
    );
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_checkout,
        cancel,
        delete_all_orders,
        delete_order_by_id,
        get_all_orders_for_user,
        get_order_by_id,
        success,
    ),
    components(
        schemas(
            CreateOrderHttpRequestBody,
            OrderResponseData
        )
    )
)]
pub struct ApiDoc;