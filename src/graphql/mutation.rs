use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_graphql::{Context, Error, Object, Result};
use rand_core::OsRng;
use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::{self, AuthUser};

use super::types::{Comic, ComicRow};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Create a user (dev-friendly version).
    /// NOTE: In real life you'd enforce uniqueness, validate email, etc.
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| Error::new(format!("Password hashing failed: {e}")))?
            .to_string();

        let id = Uuid::new_v4();

        let row = sqlx::query!(
            r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id::text AS "id!"
            "#,
            id,
            email,
            password_hash
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::new(format!("DB error: {e}")))?;

        Ok(row.id)
    }

    /// Login → returns JWT token string
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> Result<String> {
        let pool = ctx.data::<PgPool>()?;
        let jwt_secret = ctx
            .data::<String>()
            .map_err(|_| Error::new("JWT secret missing from schema data"))?;

        let row = sqlx::query!(
            r#"
            SELECT id::text as "id!", password_hash
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::new(format!("DB error: {e}")))?;

        let row = row.ok_or_else(|| Error::new("Invalid email or password"))?;

        let parsed_hash = PasswordHash::new(&row.password_hash)
            .map_err(|_| Error::new("Stored password hash is invalid"))?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::new("Invalid email or password"))?;

        let token = auth::sign_token(&row.id, jwt_secret)
            .map_err(|e| Error::new(format!("Token signing failed: {e}")))?;

        Ok(token)
    }

    /// Protected: create comic (author comes from JWT)
    async fn create_comic(
        &self,
        ctx: &Context<'_>,
        title: String,
        description: String,
        image_url: Option<String>,
    ) -> Result<Comic> {
        let pool = ctx.data::<PgPool>()?;

        let user = ctx
            .data_opt::<AuthUser>()
            .cloned()
            .ok_or_else(|| Error::new("Not authenticated (missing/invalid Bearer token)"))?;

        let comic_id = Uuid::new_v4();

        // IMPORTANT:
        // Your AuthUser.user_id is already a Uuid → do NOT parse it.
        let author_id: Uuid = user.user_id;

        let row = sqlx::query_as!(
            ComicRow,
            r#"
            INSERT INTO comics (id, author_id, title, description, image_url)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
              id::text          AS "id!",
              author_id::text   AS "author_id!",
              title             AS "title!",
              description       AS "description!",
              image_url         AS "image_url?",
              created_at::text  AS "created_at!"
            "#,
            comic_id,
            author_id,
            title,
            description,
            image_url
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::new(format!("DB error: {e}")))?;

        Ok(Comic::from(row))
    }
}
