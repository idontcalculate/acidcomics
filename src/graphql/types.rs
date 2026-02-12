use async_graphql::SimpleObject;

/// GraphQL type exposed to the client.
/// We keep IDs + timestamps as String to avoid extra scalar plumbing.
#[derive(SimpleObject, Clone, Debug)]
pub struct Comic {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub created_at: String,
}

/// DB row used by sqlx::query_as!
/// IMPORTANT: field names and types must match the SQL SELECT/RETURNING exactly.
#[derive(Clone, Debug)]
pub struct ComicRow {
    pub id: String,
    pub author_id: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub created_at: String,
}

impl From<ComicRow> for Comic {
    fn from(r: ComicRow) -> Self {
        Self {
            id: r.id,
            author_id: r.author_id,
            title: r.title,
            description: r.description,
            image_url: r.image_url,
            created_at: r.created_at,
        }
    }
}
