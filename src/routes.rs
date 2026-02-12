use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::{HeaderMap, StatusCode},
    response::Html,
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;

use crate::{
    auth,
    graphql::schema::{build_schema, AppSchema},
};

pub fn router(pool: PgPool, jwt_secret: String) -> Router {
    let schema = build_schema(pool, jwt_secret);

    Router::new()
        .route("/health", get(health))
        .route("/graphql", post(graphql_handler))
        .route("/graphiql", get(graphiql))
        .layer(Extension(schema))
}

async fn health() -> &'static str {
    "AcidComics™ backend alive 🦀"
}

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut gql_req = req.into_inner();

    // If Authorization header exists, try to attach AuthUser into request context
    if let Some(authz) = headers.get(axum::http::header::AUTHORIZATION) {
        if let Ok(s) = authz.to_str() {
            if let Some(token) = s
                .strip_prefix("Bearer ")
                .or_else(|| s.strip_prefix("bearer "))
            {
                // We don't fail the whole request on bad tokens here;
                // resolvers that require auth will check for AuthUser presence.
                if let Some(jwt_secret) = schema.data::<String>() {
                    if let Ok(user) = auth::verify_token(token, jwt_secret) {
                        gql_req = gql_req.data(user);
                    }
                }
            }
        }
    }

    schema.execute(gql_req).await.into()
}

async fn graphiql() -> Html<String> {
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint("/graphql")
            .finish(),
    )
}
