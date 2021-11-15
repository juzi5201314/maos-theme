
use seed::app::{CmdHandle, Orders};
use seed::prelude::*;

use seed::{a, article, attrs, div, document, h1, h2, h4, nodes, p, strong, Url, C, md};

use crate::{Msg, Page};
use crate::data_model::{SiteConfig, PostModel};

pub struct Model {
    raw_data: Data,
    url: Url,
}

impl Model {
    pub fn view(&self) -> Vec<Node<Msg>> {
        let data = self.raw_data.clone();
        document().set_title(&data.site.title);
        nodes![
            div![
                C!["uk-flex", "uk-heading-divider", "uk-flex-bottom"],
                h1![C!["uk-heading-medium"], data.site.name],
                h4![
                    C!["uk-margin-right"],
                    if data.logged {
                        a![
                            C!["uk-link-reset"],
                            ev(Ev::Click, |_| Msg::Logout),
                            strong!("logout")
                        ]
                    } else {
                        a![
                            C!["uk-link-reset"],
                            attrs! {
                                At::Href => self
                                    .url
                                    .to_hash_base_url()
                                    .add_hash_path_part("login")
                                    .to_string()
                            },
                            strong!("login")
                        ]
                    }
                ]
            ],
            data.posts
                .into_iter()
                .map(|post| {
                    article![
                        h2![
                            C!["uk-h2", "uk-heading-bullet"],
                            a![
                                C!["uk-link-reset"],
                                attrs! {
                                    At::Href => self
                                        .url
                                        .to_hash_base_url()
                                        .add_hash_path_part("post")
                                        .add_hash_path_part(post.id.to_string())
                                        .to_string()
                                },
                                post.title
                            ]
                        ],
                        p![
                            C!["uk-article-meta"],
                            post.create_time.format("%Y-%m-%d %H:%M").to_string()
                        ],
                        p![
                            C!["post_content"],
                            md!(&truncate(&post.content, 100))
                        ]
                    ]
                })
                .collect::<Vec<Node<Msg>>>()
        ]
    }
}

pub fn init(url: &mut Url, orders: &mut impl Orders<Msg>) -> CmdHandle {
    let url = url.clone();
    orders.perform_cmd_with_handle(async move {
        let data = crate::fetch::get::<Data>("/api").await.unwrap();
        Msg::Render(Page::Index(Model {
            url,
            raw_data: data,
        }))
    })
}

fn truncate(s: &str, max: usize) -> String {
    match s.char_indices().nth(max) {
        None => s.to_owned(),
        Some((idx, _)) => format!("{} ...", &s[..idx]),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Data {
    site: SiteConfig,
    logged: bool,
    posts: Vec<PostModel>,
}

