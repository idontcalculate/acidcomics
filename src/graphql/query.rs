use async_graphql::{Context, Error, Object, Result};
use sqlx::PgPool;

use crate::auth::{me_from_auth_user, AuthUser, Me};

use super::types::{Comic, ComicRow};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn comics(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 20)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> Result<Vec<Comic>> {
        let pool = ctx.data::<PgPool>()?;

        let limit: i64 = limit as i64;
        let offset: i64 = offset as i64;

        let rows = sqlx::query_as!(
            ComicRow,
            r#"
            SELECT
              id::text          AS "id!",
              author_id::text   AS "author_id!",
              title             AS "title!",
              description       AS "description!",
              image_url         AS "image_url?",
              created_at::text  AS "created_at!"
            FROM comics
            ORDER BY created_at DESC
            LIMIT $1
            OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::new(format!("DB error: {e}")))?;

        Ok(rows.into_iter().map(Comic::from).collect())
    }

    async fn me(&self, ctx: &Context<'_>) -> Result<Me> {
        let user = ctx
            .data_opt::<AuthUser>()
            .ok_or_else(|| Error::new("Not authenticated (missing/invalid Bearer token)"))?;

        Ok(me_from_auth_user(user))
    }
}
