use seed::app::{Orders, CmdHandle};
use seed::virtual_dom::Node;
use seed::{plain, Url, nodes};

use crate::{Msg, ViewModel};
use chrono::NaiveDateTime;

pub struct Model {
    raw_data: Data
}

impl ViewModel for Model {
    fn view(&self) -> Vec<Node<Msg>> {
        nodes![
            plain!("index"),
            plain!(format!("{:#?}", &self.raw_data))
        ]
    }
}

pub fn init(_url: &mut Url, orders: &mut impl Orders<Msg>) -> CmdHandle {
    orders.perform_cmd_with_handle(async move {
        let data = crate::fetch::get::<Data>("/api").await.unwrap();
        Msg::Render(Some(box Model {
            raw_data: data
        }))
    })
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Data {
    site: SiteConfig,
    logged: bool,
    posts: Vec<PostModel>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SiteConfig {
    name: String,
    title: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PostModel {
    pub id: u32,
    pub title: String,
    pub content: String,

    pub create_time: NaiveDateTime,
    pub last_modified_time: NaiveDateTime,
}
