use crate::domain::models::order::{CreateOrderError, DeleteOrderError, FindOrderError, Order, UpdateOrderError};
use crate::domain::models::order_details::{SessionId, SessionStatus, UserName};
use crate::domain::models::order_item::OrderItem;
use crate::domain::ports::order_repository::OrderRepository;
use crate::outbound::entities::order_details::FetchOrderDetailsEntity;
use crate::outbound::entities::order_details::{CreateOrderDetailsEntity, SessionStatusEntity};
use crate::outbound::entities::order_item::{CreateOrderItemEntity, FetchOrderItemEntity};
use anyhow::{anyhow, Context};
use rust_decimal::Decimal;
use sqlx::postgres::PgConnectOptions;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{Executor, PgPool, Transaction};
use std::str::FromStr;
use getset::Getters;
use uuid::Uuid;

#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
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

    async fn find_details_by_session_id(
        &self,
        session_id: &SessionId,
    ) -> Result<FetchOrderDetailsEntity, sqlx::Error> {
        let details = sqlx::query_as!(
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

        Ok(details)

    }
    async fn find_order_details_by_username(&self,
                                                username: &UserName
    ) -> Result<Vec<FetchOrderDetailsEntity>, sqlx::Error> {
        let details: Vec<FetchOrderDetailsEntity> = sqlx::query_as!(
            FetchOrderDetailsEntity,
            r#"
            SELECT id,
                   username,
                   status AS "status: SessionStatusEntity",
                   session_id,
                   created_at AS "created_at: DateTime<Utc>"
            FROM order_details
            WHERE username = $1
            "#,
            username.to_string()
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(details)
    }
    async fn find_order_items_by_order_id(
        &self,
        order_id: &Uuid,
    ) -> Result<Vec<FetchOrderItemEntity>, sqlx::Error> {

        let items: Vec<FetchOrderItemEntity> = sqlx::query_as!(
            FetchOrderItemEntity,
            r#"
            SELECT id,
                   product_name,
                   item_id,
                   price AS "price: Decimal",
                   order_id
            FROM order_item
            WHERE order_id = $1
            "#,
            order_id,
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(items)
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

    async fn create_order_items(&self,
                               items: Vec<CreateOrderItemEntity>,
                               tx: &mut Transaction<'_, sqlx::Postgres>
    ) -> Result<(), sqlx::Error> {
        let ids: Vec<Uuid> = items.iter().map(|item| item.id).collect();
        let product_names: Vec<String> = items.iter().map(|item| item.product_name.clone()).collect();
        let item_ids: Vec<Uuid> = items.iter().map(|item| item.item_id).collect();
        let prices: Vec<Decimal> = items.iter().map(|item| item.price).collect();
        let order_ids: Vec<Uuid> = items.iter().map(|item| item.order_id).collect();
        let query = sqlx::query_as!(
            CreateOrderItemEntity,
            r#"
            INSERT INTO order_item (id, product_name, item_id, price, order_id)
            SELECT * FROM UNNEST($1::Uuid[], $2::text[], $3::Uuid[], $4::Decimal[], $5::Uuid[])
            "#,
            &ids,
            &product_names,
            &item_ids,
            &prices,
            &order_ids,
        );
        tx.execute(query).await?;

        Ok(())
    }

    async fn delete_orders(&self) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM order_details")
            .execute(&self.pool)
            .await?;

        Ok(())

    }
    
    async fn find_details_by_id(&self, id: &Uuid) -> Result<FetchOrderDetailsEntity, sqlx::Error> {
        let details: FetchOrderDetailsEntity = sqlx::query_as!(
            FetchOrderDetailsEntity,
            r#"
            SELECT id,
                   username,
                   status AS "status: SessionStatusEntity",
                   session_id,
                   created_at AS "created_at: DateTime<Utc>"
            FROM order_details
            WHERE id = $1
            "#,
            id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(details) 
    }
    
    async fn process_details(&self, details: FetchOrderDetailsEntity) 
        -> Result<Order, FindOrderError> {
        let items: Vec<FetchOrderItemEntity> = self.find_order_items_by_order_id(&details.id)
            .await
            .map_err(|e| {
                FindOrderError::Unknown(anyhow!(e).context(format!(
                    "Error finding order items for order id {}"
                    ,details.id
                )))
            })?;
        let details = details.into_domain();
        let items: Vec<OrderItem> = items
            .into_iter()
            .map(|item| {
                let id = item.id.clone();

                item.try_into_domain().map_err(|e| {
                    FindOrderError::Unknown(anyhow!(e).context(format!(
                        "Failed to convert order item with id {} into domain",
                        id
                    )))
                })
            })
            .collect::<Result<Vec<_>, FindOrderError>>()?;

        let order = Order::new(details, items).map_err(|e| {
            FindOrderError::Unknown(anyhow!(e).context("Failed to convert to order!".to_string()))
        })?;

        Ok(order)
    }

    async fn update_order_details_status(
        &self,
        id: &Uuid,
        status: Option<SessionStatusEntity>,
    ) -> Result<FetchOrderDetailsEntity, sqlx::Error> {
        let updated_details = sqlx::query_as!(
            FetchOrderDetailsEntity,
            r#"
            UPDATE order_details
            SET status = $1
            WHERE id = $2
            RETURNING id, username, status as "status: SessionStatusEntity",
            session_id,
            created_at as "created_at: DateTime<Utc>"
            "#,
            status as Option<SessionStatusEntity>,
            id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(updated_details)
    }
}

impl OrderRepository for Postgres {
    
    async fn find_order_by_session_id(&self, req: &SessionId) -> Result<Order, FindOrderError> {
        let details = self.find_details_by_session_id(req)
            .await
            .map_err(|e| {
                FindOrderError::Unknown(anyhow!(e).context(format!(
                    "Error finding order details by session {req}"
                )))
            })?;

        self.process_details(details).await
    }

    async fn find_orders_by_username(&self, req: &UserName) -> Result<Vec<Order>, FindOrderError> {
        let mut orders: Vec<Order> = Vec::new();

        let details_for_name = self.find_order_details_by_username(req)
            .await
            .map_err(|e|
            FindOrderError::Unknown(anyhow!(e).context(format!(
                "Error finding order details for username {req}"
            ))))?;

        for details in details_for_name {
            let order = self.process_details(details).await?;
            orders.push(order);
        }
        
        Ok(orders)

    }

    async fn create_order(&self, req: &Order) -> Result<Uuid, CreateOrderError> {
        let order_details = CreateOrderDetailsEntity::from_domain(req.details());
        let order_id = order_details.id.clone();
        let mut tx = self
            .pool
            .begin()
            .await
            .context("failed to start Postgres transaction")?;

        self.create_order_details(order_details, &mut tx)
            .await
            .map_err(|e| {
            CreateOrderError::Unknown(anyhow!(e).context(format!(
                "failed to save order with id {:?}",
                req.details().order_id()
            )))
        })?;


        let items: Vec<CreateOrderItemEntity> = req
            .items()
            .into_iter()
            .map(|item| CreateOrderItemEntity::from_domain(item, &order_id))
            .collect();



        self.create_order_items(items, &mut tx)
            .await
            .context("failed to create order items")?;



        tx.commit().await.context("failed to commit transaction")?;

        Ok(order_id)

    }

    async fn delete_order(&self, req: Uuid) -> Result<Uuid, DeleteOrderError> {
        self.delete_order_by_id(req)
            .await
            .map_err(|e| {
            DeleteOrderError::Unknown(anyhow!(e).context(format!(
                "failed to delete order with ID {:?}", req.clone()
            )))
        })?;
        Ok(req)
    }

    async fn delete_all_orders(&self) -> Result<(), DeleteOrderError> {
        self.delete_orders().await.map_err(|e| {
            DeleteOrderError::Unknown(anyhow!(e).context("failed to delete all orders.".to_string()))
        })
    }

    async fn find_order_by_id(&self, req: Uuid) -> Result<Order, FindOrderError> {
        let details = self.find_details_by_id(&req)
            .await
            .map_err(|e| {
                FindOrderError::Unknown(anyhow!(e).context(format!(
                    "Error finding order details by session {req}"
                )))
            })?;
        
        
        self.process_details(details).await
        
    }

    async fn update_order_status(
        &self, id: &Uuid,
        status: Option<&SessionStatus>
    ) -> Result<Order, UpdateOrderError> {
        let status_entity = status.map(|s| SessionStatusEntity::from(s.clone()));
        let updated_details = self
            .update_order_details_status(id, status_entity)
            .await
            .map_err(|e| {
                UpdateOrderError::Unknown(anyhow!(e).context(format!(
                    "Failed to update order details with id {id}"
                )))
            })?;

        self.process_details(updated_details)
            .await
            .map_err(|e| {
            UpdateOrderError::Unknown(anyhow!(e).context(format!(
                "Failed to process order details with id {id}"
            )))
        })
    }
}