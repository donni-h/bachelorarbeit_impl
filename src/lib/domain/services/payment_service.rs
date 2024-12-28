use crate::domain::models::order::CreateOrderRequest;
use crate::domain::models::order_details::{SessionId, SessionStatus};
use crate::domain::ports::payment_service::{PaymentService, PaymentServiceError};
use std::str::FromStr;
use anyhow::anyhow;
use stripe::{CheckoutSession, CheckoutSessionBillingAddressCollection, CheckoutSessionId, CheckoutSessionMode, CheckoutSessionStatus, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCheckoutSessionLineItemsPriceData, CreateCheckoutSessionLineItemsPriceDataProductData, CreateCheckoutSessionPaymentMethodTypes, CreateCheckoutSessionShippingAddressCollection, CreateCheckoutSessionShippingAddressCollectionAllowedCountries, Currency, StripeError};
use thiserror::Error;
use crate::domain::models::order_item::OrderItem;

#[derive(Clone)]
pub struct StripeService {
    client: Client,
    redirect_url: String,
}

impl StripeService {
    pub fn new(secret: String, redirect_url: String) -> Self {
        let client = Client::new(secret);
        
        Self { client, redirect_url }
    }
}

impl PaymentService for StripeService {
    async fn create_checkout_session(&self, order_items: &Vec<OrderItem>) -> Result<CheckoutSession, PaymentServiceError> {
        let allowed_countries = vec![
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::De,
            CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Us,
        ];
        let success_url = format!("{}/success?session_id={{CHECKOUT_SESSION_ID}}", self.redirect_url);
        let cancel_url = format!("{}/cancel?session_id={{CHECKOUT_SESSION_ID}}", self.redirect_url);

        let line_items: Vec<CreateCheckoutSessionLineItems> = order_items
            .iter()
            .map(|item| item.to_stripe_line_item())
            .collect();

        let params = CreateCheckoutSession {
            billing_address_collection: Some(CheckoutSessionBillingAddressCollection::Required),
            shipping_address_collection: Some(CreateCheckoutSessionShippingAddressCollection {
                allowed_countries,
            }),
            payment_method_types: Some(vec![CreateCheckoutSessionPaymentMethodTypes::Card]),
            mode: Some(CheckoutSessionMode::Payment),
            success_url: Some(success_url.as_str()),
            cancel_url: Some(cancel_url.as_str()),
            line_items: Some(line_items),
            ..Default::default()
        };

        CheckoutSession::create(&self.client, params)
            .await
            .map_err(PaymentServiceError::from)

    }

    async fn retrieve_checkout_status(&self, id: &SessionId) -> Result<Option<SessionStatus>, PaymentServiceError> {
        let session_id = CheckoutSessionId::from_str(&id.to_string())
            .map_err(|e| {
                PaymentServiceError::InvalidSessionId(id.clone())
            })?;

        let checkout_session = CheckoutSession::retrieve(&self.client, &session_id, &[])
            .await
            .map_err(|e| {
                PaymentServiceError::Unknown(anyhow!(e).context(format!(
                "Failed to retrieve checkout session with id {}",
                id
            )))
            })?;

        let status = checkout_session.status.map(SessionStatus::from);

        Ok(status)
    }

    async fn expire_session(&self, id: &SessionId) -> Result<(), PaymentServiceError> {
        let session_id = CheckoutSessionId::from_str(&id.to_string())
            .map_err(|e| {
                PaymentServiceError::InvalidSessionId(id.clone())
            })?;
        let _ = CheckoutSession::expire(&self.client, &session_id).await.map_err(|e| {
            PaymentServiceError::Unknown(anyhow!(e).context(format!(
                "Failed to expire checkout session with id {}",
                id
            )))
        })?;
        
        Ok(())
    }
}

impl From<CheckoutSessionStatus> for SessionStatus {
    fn from(status: CheckoutSessionStatus) -> Self {
        match status {
            CheckoutSessionStatus::Complete => SessionStatus::Complete,
            CheckoutSessionStatus::Expired => SessionStatus::Expired,
            CheckoutSessionStatus::Open => SessionStatus::Open,
        }
    }
}

impl OrderItem {
    fn to_stripe_line_item(&self) -> CreateCheckoutSessionLineItems {
        CreateCheckoutSessionLineItems {
            quantity: Some(1),
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                currency: Currency::EUR,

                unit_amount: self.price().as_cents(),
                product_data: Some(CreateCheckoutSessionLineItemsPriceDataProductData {
                    name: self.product_name().to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

impl From<StripeError> for PaymentServiceError {
    fn from(err: StripeError) -> Self {
        PaymentServiceError::Unknown(anyhow!(
            "Failed to create a Stripe checkout session: {:?}",
            err
        ))
    }
}