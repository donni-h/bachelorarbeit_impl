use crate::domain::ports::checkout_producer::CheckoutProducer;

#[derive(Debug, Clone)]
pub struct RabbitMQ;


impl RabbitMQ {
    pub fn new() -> Self {
        Self
    }
}

impl CheckoutProducer for RabbitMQ {
    
}

