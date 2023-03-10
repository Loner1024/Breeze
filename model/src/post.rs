use serde::{Deserialize, Serialize};

#[derive(Serialize, PartialEq, Deserialize)]
pub struct PostList {
    pub count: usize,
    pub data: Vec<PostListItem>,
}

#[derive(Serialize, PartialEq, Deserialize, Clone)]
pub struct PostListItem {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub create_time: i64,
}

#[derive(Serialize, Deserialize, PartialEq, Default)]
pub struct PostDetail {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub create_time: i64,
}