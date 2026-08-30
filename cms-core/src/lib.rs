use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body_html: String,
    pub status: String,
}
