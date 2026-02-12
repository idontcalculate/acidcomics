use async_graphql::{EmptySubscription, Schema};
use sqlx::PgPool;

use super::mutation::MutationRoot;
use super::query::QueryRoot;

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn build_schema(pool: PgPool, jwt_secret: String) -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool)
        .data(jwt_secret)
        .finish()
}
