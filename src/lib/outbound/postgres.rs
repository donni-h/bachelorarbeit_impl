use std::future::Future;
use std::str::FromStr;
use anyhow::{anyhow, Context};
use sqlx::{query_as, Executor, PgPool, Transaction};
use sqlx::postgres::PgConnectOptions;
use uuid::Uuid;
use crate::domain::models::order_details::{OrderDetails, SessionId, UserName};
use crate::domain::ports::order_repository::OrderRepository;
use crate::domain::models::order::{CreateOrderError, DeleteOrderError, FindOrderError, Order};
use crate::outbound::entities::order_details::{CreateOrderDetailsEntity, SessionStatusEntity};
use sqlx::types::chrono::{DateTime, Utc};
use crate::outbound::entities::order_details::FetchOrderDetailsEntity;
use crate::outbound::entities::order_item::CreateOrderItemEntity;

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

    async fn delete_order_by_id(&self, id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM order_details
            WHERE id = $1
            "#,
        id
        )
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }

    pub async fn find_metadata_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<FetchOrderDetailsEntity, sqlx::Error> {
        let metadata = sqlx::query_as!(
            FetchOrderDetailsEntity,
            r#"
            SELECT id,
                   username,
                   status AS "status: SessionStatusEntity",
                   session_id,
                   created_at AS "created_at: DateTime<Utc>"
            FROM order_details
            WHERE session_id = $1
            "#,
            session_id.to_string()
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(metadata)

    }

    async fn create_order_details(&self,
                                      details: CreateOrderDetailsEntity,
                                      tx: &mut Transaction<'_, sqlx::Postgres>
    ) -> Result<(), sqlx::Error> {
        let query = sqlx::query_as!(
            CreateOrderDetailsEntity,
            r#"
            INSERT INTO order_details (id, username, status, session_id, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            details.id,
            details.username,
            details.status as _,
            details.session_id,
            details.created_at as DateTime<Utc>,
        );
        tx.execute(query).await?;

        Ok(())

    }

    async fn create_order_item(&self,
                               item: CreateOrderItemEntity,
                               tx: &mut Transaction<'_, sqlx::Postgres>
    ) -> Result<(), sqlx::Error> {
        let query = sqlx::query_as!(
            CreateOrderItemEntity,
            r#"
            INSERT INTO order_item (id, product_name, item_id, price, order_id)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            item.id,
            item.product_name,
            item.item_id,
            item.price,
            item.order_id,
        );
        tx.execute(query).await?;

        Ok(())
    }
}

impl OrderRepository for Postgres {

    async fn create_order(&self, req: &Order) -> Result<Uuid, CreateOrderError> {
        let order_details = CreateOrderDetailsEntity::from_domain(req.order_details());
        let order_id = &order_details.id.clone();
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start Postgres transaction")?;

        self.create_order_details(order_details, &mut tx).await.map_err(|e| {
            CreateOrderError::Unknown(anyhow!(e).context(format!(
                "failed to save order with id {:?}",
                req.order_details().order_id()
            )))
        })?;


        let items: Vec<CreateOrderItemEntity> = req
            .items()
            .into_iter()
            .map(|item| CreateOrderItemEntity::from_domain(item, order_id))
            .collect();


        for item in items {
            self.create_order_item(item, &mut tx)
                .await
                .context("failed to create order item")?;
        }


        tx.commit().await.context("failed to commit transaction")?;

        Ok(order_id.clone())

    }

    async fn delete_order(&self, req: Uuid) -> Result<Uuid, DeleteOrderError> {
        self.delete_order_by_id(req).await.map_err(|e| {
            DeleteOrderError::Unknown(anyhow!(e).context(format!(
                "failed to delete order with ID {:?}", req.clone()
            )))
        })?;
        Ok(req)
    }
}