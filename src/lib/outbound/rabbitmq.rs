use std::future::Future;
use std::sync::mpsc::channel;
use amqprs::BasicProperties;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicPublishArguments, Channel, ExchangeDeclareArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use serde::Serialize;
use crate::domain::models::order_details::{SessionStatus, UserName};
use crate::domain::ports::checkout_producer::{CheckoutProducer, NotifyError};

#[derive(Clone)]
pub struct RabbitMQ {
    channel: Channel,
    exchange: String,
    routing_key: String,
}

#[derive(Serialize)]
struct CheckoutResult {
    username: String,
    status: String,
}

impl RabbitMQ {
    pub async fn new(host: &str, port: u16, routing_key: &str, exchange_name: &str) -> Self {
        let connection = Connection::open(
            &OpenConnectionArguments::new(
                host,
                port,
                "admin",
                "admin",
            )
        )
            .await
            .expect("Failed to open connection");


        let channel = connection.open_channel(None)
            .await
            .expect("Failed to open channel");

        let queue_name = "Checkout_ToBasket";
        channel
            .queue_declare(QueueDeclareArguments::durable_client_named(queue_name))
            .await
            .expect("Failed to declare queue");

        Self {
            channel,
            exchange: exchange_name.to_string(),
            routing_key: routing_key.to_string(),
        }
    }
}

impl CheckoutProducer for RabbitMQ {
    async fn notify_order_result(&self,
                                 username: &UserName,
                                 status: &SessionStatus
    ) -> Result<(), NotifyError>{
        let result = CheckoutResult {
            username: username.to_string(),
            status: status.to_string(),
        };

        let payload = serde_json::to_vec(&result).unwrap();

        let args = BasicPublishArguments::new("","Checkout_ToBasket");
        self.channel
            .basic_publish(BasicProperties::default(),
                           payload,
                           args)
            .await
            .unwrap();

        Ok(())
    }
}

