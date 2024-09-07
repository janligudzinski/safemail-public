use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_pool() -> Arc<PgPool> {
    
    sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .map(Arc::new)
            .unwrap()
}
