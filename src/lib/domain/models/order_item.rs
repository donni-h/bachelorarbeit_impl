use derive_more::{AsRef, Display, From};
use getset::Getters;
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Getters)]
#[getset(get = "pub")]
pub struct OrderItem {
    id: uuid::Uuid,
    product_name: ProductName,
    item_id: uuid::Uuid,
    price: Price,
}

impl OrderItem {
    pub fn new(id: Uuid, product_name: ProductName, item_id: uuid::Uuid, price: Price) -> Self {
        Self {id, product_name, item_id, price,}
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(raw: &str) -> Self {
        Self(raw.trim().to_string())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRef)]
pub struct Price(Decimal);

#[derive(Clone, Debug, Error)]
pub enum PriceError {
    #[error("price has to be positive")]
    Negative,
    #[error("cannot be represented with this input")]
    Unrepresentable,
}

impl Price {
    pub fn new(raw: f64) -> Result<Self, PriceError> {
        let value = Decimal::from_f64(raw).ok_or(PriceError::Unrepresentable)?;
        if value <= Decimal::ZERO {
            Err(PriceError::Negative)
        } else {
            Ok(Self(value))
        }

    }

    pub fn as_cents(&self) -> Option<i64> {
        (self.0 * Decimal::new(100, 0)).to_i64()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, From, Getters)]
#[getset(get = "pub")]
pub struct CreateOrderItemRequest {
    id: Uuid,
    product_name: ProductName,
    item_id: Uuid,
    price: Price,
}

impl CreateOrderItemRequest {
    pub fn new(product_name: ProductName, item_id: uuid::Uuid, price: Price) -> Self {
        Self {id: Uuid::new_v4(), product_name, item_id, price}
    }
}