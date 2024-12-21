use sqlx::FromRow;
use sqlx::types::uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct OrderEntity {
    pub id: Uuid,

}
