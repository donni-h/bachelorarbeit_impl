use std::future::Future;
use std::str::FromStr;
use anyhow::{anyhow, Context};
use sqlx::{query_as, PgPool, Transaction};
use sqlx::postgres::PgConnectOptions;
use uuid::Uuid;
use crate::domain::models::metadata::{Metadata, SessionId, UserName};
use crate::domain::ports::order_repository::OrderRepository;
use crate::domain::models::order::{CreateOrderError, DeleteOrderError, FindOrderError, Order};
use crate::outbound::entities::metadata::MetadataEntity;
use crate::outbound::entities::metadata::SessionStatus;

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn new(path: &str) -> Result<Self, anyhow::Error> {
        let pool = PgPool::connect_with(
            PgConnectOptions::from_str(path)
                .with_context(|| format!("invalid database path {}", path))?
        )
            .await
            .with_context(|| format!("failed to open database at {}", path))?;

        Ok(Self { pool })
    }

    async fn find_metadata_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<MetadataEntity, sqlx::Error> {
        let metadata = sqlx::query_as!(
            MetadataEntity,
            r#"
            SELECT id,
                   order_id,
                   username,
                   status AS "status: SessionStatus",
                   session_id,
                   created_at
            FROM metadata
            WHERE session_id = $1
            "#,
            session_id.to_string()
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(metadata)

    }
}

/*impl OrderRepository for Postgres {
    async fn find_order_by_session_id(&self, req: &SessionId) -> impl Future<Output=Result<Order, FindOrderError>> + Send {
        todo!()
    }

    fn find_orders_by_username(&self, req: &UserName) -> impl Future<Output=Result<Vec<Order>, FindOrderError>> + Send {
        todo!()
    }

    fn create_order(&self, req: &Order) -> impl Future<Output=Result<Order, CreateOrderError>> + Send {
        todo!()
    }

    fn delete_order(&self, req: Uuid) -> impl Future<Output=Result<uuid::Uuid, DeleteOrderError>> + Send {
        todo!()
    }

    fn delete_all_orders(&self) -> impl Future<Output = Result<(), DeleteOrderError>> {
        todo!()
    }
}*/