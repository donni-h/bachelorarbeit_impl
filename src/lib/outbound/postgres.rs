use std::future::Future;
use std::str::FromStr;
use anyhow::{anyhow, Context};
use sqlx::{query_as, PgPool, Transaction};
use sqlx::postgres::PgConnectOptions;
use uuid::Uuid;
use crate::domain::models::metadata::{Metadata, SessionId, UserName};
use crate::domain::ports::order_repository::OrderRepository;
use crate::outbound::entities::order::{OrderQueryResult};
use crate::domain::models::order::FindOrderError;
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

    async fn find_by_session_id(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        session_id: &SessionId,
    ) -> Result<OrderQueryResult, sqlx::Error> {
        let order = query_as!(
            OrderQueryResult,
            r#"
            SELECT
                o.id AS id,
                m.id AS metadata_id,
                m.order_id AS metadata_order_id,
                m.username AS metadata_username,
                m.status AS "metadata_status: SessionStatus",
                m.session_id AS metadata_session_id,
                m.created_at AS metadata_created_at,
                ARRAY_AGG((oi.id, oi.product_name, oi.item_id, oi.price, oi.order_id)) AS "order_items: Vec<OrderItemEntity>"
            FROM orders o
            LEFT JOIN metadata m ON o.id = m.order_id
            LEFT JOIN order_item oi ON o.id = oi.order_id
            WHERE m.session_id = $1
            GROUP BY o.id, m.id, m.order_id, m.username, m.status, m.session_id, m.created_at;
            "#,
            session_id.to_string()
        )
            .fetch_one(tx)
            .await?;

        Ok(order)

    }
}

impl OrderRepository for Postgres {
    async fn find_order_by_session_id(&self, req: &SessionId) -> impl Future<Output=Result<Order, FindOrderError>> + Send {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start Postgres transaction")?;

        let order_query = self.find_by_session_id(&mut tx, req).await.map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                FindOrderError::IdNotFound {id: req.clone().into()}
            } else{
                anyhow!(e)
                    .context(format!("failed to find order by session id {}", req))
                    .into()
            }
        })?;
        tx.commit()
            .await
            .context("failed to commit Postgres transaction")?;

        let metadata_id = order_query.metadata_id;
        let username = order_query.metadata_status;
        let metadata_session_id = order_query.metadata_session_id;
        let metadata = Metadata::new()

    }

    fn find_orders_by_username(&self, req: &UserName) -> impl Future<Output=Result<Vec<Order>, FindOrderError>> + Send {
        todo!()
    }

    fn create_order(&self, req: &Order) -> impl Future<Output=Result<Order, CreateOrderError>> + Send {
        todo!()
    }

    fn delete_order(&self, req: Uuid) -> impl Future<Output=Result<(), DeleteOrderError>> + Send {
        todo!()
    }

    fn delete_all_orders(&self) -> impl Future<Output=Result<(), DeleteOrderError>> + Send {
        todo!()
    }
}