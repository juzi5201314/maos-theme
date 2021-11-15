use chrono::NaiveDateTime;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct SiteConfig {
    pub name: String,
    pub title: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct PostModel {
    pub id: u32,
    pub title: String,
    pub content: String,

    pub create_time: NaiveDateTime,
    pub last_modified_time: NaiveDateTime,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct CommentModel {
    pub id: u32,

    pub post_id: u32,
    pub content: String,
    pub create_time: NaiveDateTime,

    pub email: String,

    pub nickname: String,

    pub parent_id: Option<u32>,

    pub deleted: bool,
}
