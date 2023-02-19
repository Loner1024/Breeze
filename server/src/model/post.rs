use serde::Serialize;

#[derive(Serialize)]
pub struct PostList {
    pub count: usize,
    pub data: Vec<PostListItem>,
}

#[derive(Serialize)]
pub struct PostListItem {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub create_time: i64,
}