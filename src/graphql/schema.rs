use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

use super::mutation::MutationRoot;
use super::query::QueryRoot;

/// Wrapper type so we don't store a plain String in Schema data.
/// This avoids collisions if we later store other strings in Schema.
#[derive(Clone)]
pub struct JwtSecret(pub String);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(pool: PgPool, jwt_secret: String) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .data(JwtSecret(jwt_secret))
        .finish()
}
