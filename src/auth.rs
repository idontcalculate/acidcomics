use anyhow::{anyhow, Result};
use async_graphql::SimpleObject;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
}

#[derive(SimpleObject, Clone, Debug)]
pub struct Me {
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    // subject = user id
    sub: String,
    exp: usize,
}

pub fn sign_token(user_id: &str, jwt_secret: &str) -> Result<String> {
    // 7 days
    let exp = (chrono::Utc::now() + chrono::Duration::days(7)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str, jwt_secret: &str) -> Result<AuthUser> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )?;

    let user_id =
        Uuid::parse_str(&data.claims.sub).map_err(|_| anyhow!("Invalid UUID in token"))?;

    Ok(AuthUser { user_id })
}

pub fn me_from_auth_user(user: &AuthUser) -> Me {
    Me {
        user_id: user.user_id.to_string(),
    }
}
